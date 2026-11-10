use loco_rs::model::query::PageResponse;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const MAX_PAGE_SIZE: u64 = 100;
pub const DEFAULT_PAGE_SIZE: u64 = 25;

/// A page of results with stable pagination metadata.
#[derive(Debug, Serialize, Deserialize, TS)]
pub struct Page<T: TS> {
    pub items: Vec<T>,
    #[ts(type = "number")]
    pub page: u64,
    #[ts(type = "number")]
    pub page_size: u64,
    #[ts(type = "number")]
    pub total_pages: u64,
    #[ts(type = "number")]
    pub total_items: u64,
}

impl<T: TS> Page<T> {
    pub fn from_query<M>(res: PageResponse<M>) -> Self
    where
        T: From<M>,
    {
        Self {
            items: res.page.into_iter().map(T::from).collect(),
            page: res.meta.page,
            page_size: res.meta.page_size,
            total_pages: res.meta.total_pages,
            total_items: res.meta.total_items,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort: Option<String>,
    pub order: Option<SortOrder>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
    pub offset: u64,
    pub sort: String,
    pub order: SortOrder,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationError {
    PageMustBePositive,
    PageSizeOutOfRange,
    UnsupportedSortField,
    OffsetOverflow,
}

impl PaginationQuery {
    /// Normalize paging values and validate sort against the endpoint allowlist.
    /// Callers must map the returned sort key to a typed SeaORM column; never
    /// interpolate the untrusted key into SQL.
    pub fn normalize(
        &self,
        allowed_sort_fields: &[&str],
        default_sort: &str,
    ) -> Result<Pagination, PaginationError> {
        let page = self.page.unwrap_or(1);
        if page == 0 {
            return Err(PaginationError::PageMustBePositive);
        }

        let page_size = self.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        if !(1..=MAX_PAGE_SIZE).contains(&page_size) {
            return Err(PaginationError::PageSizeOutOfRange);
        }

        let sort = self.sort.as_deref().unwrap_or(default_sort);
        if !allowed_sort_fields.contains(&sort) {
            return Err(PaginationError::UnsupportedSortField);
        }

        let offset = page
            .checked_sub(1)
            .and_then(|value| value.checked_mul(page_size))
            .ok_or(PaginationError::OffsetOverflow)?;

        Ok(Pagination {
            page,
            page_size,
            offset,
            sort: sort.to_owned(),
            order: self.order.unwrap_or_default(),
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum ApiErrorCode {
    InvalidRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    Unavailable,
    InternalError,
}

impl ApiErrorCode {
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::InvalidRequest => "The request is invalid.",
            Self::Unauthorized => "Authentication is required or the credentials are invalid.",
            Self::Forbidden => "You are not allowed to perform this action.",
            Self::NotFound => "The requested resource was not found.",
            Self::Conflict => "The request conflicts with the current resource state.",
            Self::RateLimited => "Too many requests. Try again later.",
            Self::Unavailable => "The service is temporarily unavailable.",
            Self::InternalError => "An internal error occurred.",
        }
    }

    #[must_use]
    pub const fn from_status(status: u16) -> Self {
        match status {
            400 | 405 | 406 | 413 | 415 | 422 => Self::InvalidRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            409 => Self::Conflict,
            429 => Self::RateLimited,
            502..=504 => Self::Unavailable,
            _ => Self::InternalError,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
    pub request_id: Option<String>,
}

impl ApiError {
    #[must_use]
    pub fn for_status(status: u16, request_id: Option<String>) -> Self {
        let code = ApiErrorCode::from_status(status);
        Self {
            code,
            message: code.message().to_owned(),
            request_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
pub struct ApiErrorEnvelope {
    pub error: ApiError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_defaults_are_stable_and_bounded() {
        let page = PaginationQuery::default()
            .normalize(&["created_at", "name"], "created_at")
            .expect("valid defaults");
        assert_eq!(page.page, 1);
        assert_eq!(page.page_size, DEFAULT_PAGE_SIZE);
        assert_eq!(page.offset, 0);
        assert_eq!(page.sort, "created_at");
        assert_eq!(page.order, SortOrder::Desc);
    }

    #[test]
    fn pagination_rejects_zero_and_oversized_pages() {
        assert_eq!(
            PaginationQuery { page: Some(0), ..Default::default() }
                .normalize(&["created_at"], "created_at"),
            Err(PaginationError::PageMustBePositive)
        );
        assert_eq!(
            PaginationQuery { page_size: Some(101), ..Default::default() }
                .normalize(&["created_at"], "created_at"),
            Err(PaginationError::PageSizeOutOfRange)
        );
        assert_eq!(
            PaginationQuery { page_size: Some(0), ..Default::default() }
                .normalize(&["created_at"], "created_at"),
            Err(PaginationError::PageSizeOutOfRange)
        );
    }

    #[test]
    fn pagination_rejects_untrusted_sort_fields_and_overflow() {
        assert_eq!(
            PaginationQuery { sort: Some("id; DROP TABLE users".to_owned()), ..Default::default() }
                .normalize(&["created_at", "name"], "created_at"),
            Err(PaginationError::UnsupportedSortField)
        );
        assert_eq!(
            PaginationQuery { page: Some(u64::MAX), ..Default::default() }
                .normalize(&["created_at"], "created_at"),
            Err(PaginationError::OffsetOverflow)
        );
    }

    #[test]
    fn status_mapping_is_stable_and_messages_are_generic() {
        let cases = [
            (400, ApiErrorCode::InvalidRequest),
            (401, ApiErrorCode::Unauthorized),
            (403, ApiErrorCode::Forbidden),
            (404, ApiErrorCode::NotFound),
            (409, ApiErrorCode::Conflict),
            (429, ApiErrorCode::RateLimited),
            (503, ApiErrorCode::Unavailable),
            (500, ApiErrorCode::InternalError),
        ];
        for (status, expected) in cases {
            let error = ApiError::for_status(status, Some("req-123".to_owned()));
            assert_eq!(error.code, expected);
            assert_eq!(error.request_id.as_deref(), Some("req-123"));
            assert!(!error.message.contains("password"));
            assert!(!error.message.contains("SQL"));
        }
    }
}

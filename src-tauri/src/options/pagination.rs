const DEFAULT_PAGE_SIZE: u32 = 100;

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub page_size: u32
}

impl Pagination {
    /// ## Description
    /// Get the the `LIMIT` and `OFFSET` terms for pagination.
    /// ## Arguments
    /// - `page`: `u32` - The current page.
    /// ## Returns
    /// `(u32, u32)` - The `LIMIT` term and the `OFFSET` term.
    pub fn limit_offset(&self, page: u32) -> (u32, u32) {
        (self.page_size, page.saturating_mul(self.page_size))
    }

    /// ## Description
    /// Get the number of pages needed to show every row.
    /// ## Arguments
    /// - `total`: `u32` - The total number of rows.
    /// ## Returns
    /// `u32` - The number of pages, counting a partly filled last page.
    pub fn total_pages(&self, total: u32) -> u32 {
        total.div_ceil(self.page_size)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page_size: DEFAULT_PAGE_SIZE }
    }
}

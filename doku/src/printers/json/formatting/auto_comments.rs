use crate::*;

/// Determines which auto-comments - _hints_, so to say - should get displayed.
///
/// Sometimes Doku is able to automatically infer certain properties about a
/// type and provide a dedicated hint such as "this field is optional". This
/// struct allows to configure whether you'd like for such comments to be
/// printed or not.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutoComments {
    /// When set, displays hints for arrays of known sizes:
    ///
    /// ```
    /// use doku::prelude::*;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     friends: [String; 3],
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     auto_comments: doku::json::AutoComments {
    ///         array_size: true,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // Must contain exactly 3 elements
    ///     "friends": [
    ///       "string",
    ///       /* ... */
    ///     ]
    ///   }
    /// "#, doc);
    /// ```
    pub array_size: bool,

    /// When set, displays hints for optional values:
    ///
    /// ```
    /// use doku::prelude::*;
    ///
    /// #[derive(Document)]
    /// struct Person {
    ///     friend: Option<String>,
    /// }
    ///
    /// let fmt = doku::json::Formatting {
    ///     auto_comments: doku::json::AutoComments {
    ///         optional: true,
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    ///
    /// let doc = doku::to_json_fmt::<Person>(&fmt);
    ///
    /// doku::assert_doc!(r#"
    ///   {
    ///     // Optional
    ///     "friend": "string"
    ///   }
    /// "#, doc);
    /// ```
    pub optional: bool,
}

impl AutoComments {
    pub fn all() -> Self {
        Self {
            array_size: true,
            optional: true,
        }
    }

    pub fn none() -> Self {
        Self {
            array_size: false,
            optional: false,
        }
    }
}

impl Default for AutoComments {
    fn default() -> Self {
        Self::all()
    }
}

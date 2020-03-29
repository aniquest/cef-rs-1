use libc::{c_void};

// Unimplemented types. Will need proper implementations if associated CEF functionality is used.
pub type CefStringMultimap = c_void; /* cef_string_multimap_t */
pub type CefPostData = c_void; /* _cef_post_data_t */
pub type CefResourceType = c_void; /* cef_resource_type_t */
pub type CefReferrerPolicy = c_void; /* cef_referrer_policy_t */
pub type CefListValue = c_void; /* _cef_list_value_t */
pub type CefUrlRequest = c_void; /* _cef_url_request_t */
pub type CefUrlRequestClient = c_void; /* _cef_urlrequest_client_t */
pub type CefDomVisitor = c_void; /* _cef_domvisitor_t */
pub type CefDictionaryValue = c_void; /* _cef_dictionary_value_t */
pub type CefLoadHandler = c_void; /* _cef_load_handler_t */
pub type CefV8Exception = c_void; /* _cef_v8exception_t */
pub type CefV8StackTrace = c_void; /* _cef_v8stack_trace_t */
pub type CefDomNode = c_void; /* _cef_domnode_t */
pub type CefTask = c_void; /* _cef_task_t */

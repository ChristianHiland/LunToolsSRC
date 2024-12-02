// Libs (NOT USING MINE REMEMBER!)
use std::fmt;                                       // Importing Fmt from std.


// Struts

// file tags
pub const file_type_images: [&str; 13] = [".jpg", ".jpeg", ".jifif", ".pjpeg", ".pjp", ".png", ".gif", ".tif", ".tiff", ".bmp", ".webp", ".svg", "eps"];
pub const file_type_program: [&str; 10] = [".py", ".js", ".java", ".cpp", ".cs", ".rs", ".rpy", ".h", ".asm", ".s"];
pub const file_type_videos: [&str; 9] = [".mp4", ".mov", ".avi", ".wmv", ".mkv", ".webm", ".flv", ".mts", ".m2ts"];
pub const file_type_audios: [&str; 8] = [".mp3", ".wav", ".aac", ".ogg", ".flac", ".aiff", ".aif", ".wma"];
pub const file_type_documents: [&str; 5] = [".pdf", ".docx", ".xlsx", ".pptx", ".txt"];
pub const file_type_archive: [&str; 4] = [".zip", ".rar", ".tar", ".gz"];
pub const file_type_database: [&str; 3] = [".db", ".sqlite", ".mdb"];
pub const file_type_config: [&str; 3] = [".ini", ".xml", ".json"];
pub const file_type_system: [&str; 3] = [".dll", ".sys", ".inf"];
pub const file_type_fontfile: [&str; 2] = [".ttf", ".otf"];
pub const file_type_abobe: [&str; 2] = [".psd", "ai"];
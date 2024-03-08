use std::path::PathBuf;

use config_template_derive::ConfigTemplate;

#[derive(ConfigTemplate)]
pub struct AppConfig {
    /// 更新服务器上的索引文件的链接，可以填写多个备用链接以缓解网络不稳定
    /// 目前支持的协议：http(s)、webdav(s)、私有协议
    ///
    /// http协议例子：
    ///   1. http://127.0.0.1:6600/index.json （走http协议）
    ///   2. https://127.0.0.1:6600/subfolder/index.json （走https协议）
    ///
    /// webdav协议：（webdav代表走http协议，webdavs代表走https协议，这样写是为了和http源做区分）
    ///   1. webdav://user:pass:127.0.0.1:80 （默认形式，webdav使用http协议）
    ///   2. webdavs://user:pass:127.0.0.1:443/subfolder （子目录形式，webdav使用https协议，注意https默认端口为443，而非80）
    ///      -------   ---- ---- --------- --- ---------
    ///         |       |    |       |      |      |
    ///         |       |    |       |      |      +------ webdav 目录（可选）
    ///         |       |    |       |      +------------- webdav 端口（注意端口不能省略，通常是80和443
    ///         |       |    |       +-------------------- webdav 主机地址
    ///         |       |    +---------------------------- webdav 密码
    ///         |       +--------------------------------- webdav 用户名
    ///         +----------------------------------------- webdav 协议，只能是webdav或者webdavs
    ///
    /// sftp源例子： （切勿直接使用Linux登录端口做sftp源，会有极大安全隐患，请使用专业sftp服务器软件！）
    ///   1. sftp://user:pass:f0:91:07:33:28:19:c5:2c:ea:e4:a9:2b:41:ce:3d:63@192.168.5.133:22/  （默认形式，工作目录本身）
    ///   2. sftp://user:pass:f0:91:07:33:28:19:c5:2c:ea:e4:a9:2b:41:ce:3d:63@192.168.5.133:22/subfolder  （相对目录形式，末尾无斜线）
    ///   3. sftp://user:pass:f0:91:07:33:28:19:c5:2c:ea:e4:a9:2b:41:ce:3d:63@192.168.5.133:22//subfolder  （绝对目录形式，末尾无斜线）
    ///      ----   ---- ---- ----------------------------------------------- ------------- -- ----------
    ///       |      |    |                           |                             |       |       |
    ///       |      |    +----  密码        主机指纹（请自行确保指纹可信）         |       |       +------- 子目录（可选）
    ///       |      +---------  用户                                               |       +--------------- 端口，通常是22
    ///       +----------------  协议名称，只能是sftp                               +----------------------- 主机地址
    ///      主机指纹支持以下格式
    ///        1. SHA1:2Fo8c/96zv32xc8GZWbOGYOlRak=  （sha1指纹格式）
    ///        2. SHA256:oQGbQTujGeNIgh0ONthcEpA/BHxtt3rcYY+NxXTxQjs=  （sha256指纹格式）
    ///        3. MD5:d3:5e:40:72:db:08:f1:6d:0c:d7:6d:35:0d:ba:7c:32  （md5十六进制冒号分隔形式）
    ///        4. d3:5e:40:72:db:08:f1:6d:0c:d7:6d:35:0d:ba:7c:32  （base64编码过的sha1指纹格式或者sha256指纹格式）
    #[default_value(asd)]
    pub urls: Vec<String>,

    /// 不显示更新完成后的提示框
    pub disable_summary_dialog: bool,

    /// 不显示更新记录提示框
    pub disable_changelogs_dialog: bool,

    /// 记录客户端版本号文件的路径
    pub version_file_path: PathBuf,

    /// 更新的起始目录
    pub base_path: PathBuf,

    /// 发生（网络）错误更新失败时，是否可以继续进入游戏
    pub allow_error: bool,

    /// 安静模式
    pub silent_mode: bool,

    /// 为http类协议设置headers
    pub http_headers: Vec<(String, String)>,

    /// http类协议的连接超时判定时间，单位毫秒
    pub http_connection_timeout: u32,


    pub http_reading_timeout: u32,
    pub http_retrying_times: u8,
    pub http_concurrent_threads: u8,
    pub http_concurrent_chunk_size: u32,
    pub http_ignore_certificate: bool,
}
fn main() {
    //Cargo有两个主要的配置：运行cargo build时采用的dev配置和运行cargo build --release的release配置
    /*
    当项目的Cargo.toml文件中没有显式增加任何[profile.*]部分的时候，Cargo会对每一个配置都采用默认设置
    如下是dev和release配置的opt-level(代码编译优化级别)设置的默认值：
    文件名：Cargo.toml
    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3
     */
}

use serenity::all::{ActivityData, Context};
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

pub async fn load_system_info(ctx: &Context) {
    const WATCH_INTERVAL: u64 = 5;      // 計測するために測る時間

    // CPUとメモリ情報をリフレッシュ
    let mut system_info = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    // CPUとメモリの計測(5秒間)
    tokio::time::sleep(Duration::from_secs(WATCH_INTERVAL)).await;

    // 情報の更新
    system_info.refresh_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::nothing()
                .with_cpu()
                .with_memory()
            )
    );

    // システム全体のメモリ使用量
    let total_mem = system_info.total_memory();
    let used_mem = system_info.used_memory();
    // メモリ全体の使用率を計算
    let mem_percentage = (used_mem as f64 / total_mem as f64) * 100.0;

    // システム全体のCPU使用率
    let used_cpu_percentage = system_info.global_cpu_usage();

    // BOTのステータスに表示する文字列の作成
    let status_message = format!(
      "Usage CPU: {:.1}%, Memory: {:.1}%",
        used_cpu_percentage,
        mem_percentage
    );

    ctx.set_activity(Some(ActivityData::watching(status_message)));
}
use rasciigraph::{plot, Config};

fn main() {
    let cities = vec![
        "Nyeri", "Mombasa", "Nairobi", "Tokyo", "Seoul", "Shibuya", "Kyoto", "Shizuoka", "Kyoto",
        "Fukuoka",
    ];

    let distances_travelled = vec![400.50, 502.31, 330.12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    println!("\n [Cities] :: {}\n", cities.join(" -> "));

    println!(
        "{}",
        plot(
            distances_travelled.into_iter().map(|d| d as f64).collect(),
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("Travelled distances (km)".to_owned())
        )
    );
}

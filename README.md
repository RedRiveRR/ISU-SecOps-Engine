# Directory Bruteforcer (dirbrute v1.0.0)

![Build Status](https://github.com/RedRiveRR/ISU-SecOps-Engine/actions/workflows/rust.yml/badge.svg)
![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macos-lightgrey)
![Maintenance](https://img.shields.io/badge/maintenance-active-green.svg)
![Code Style](https://img.shields.io/badge/code%20style-strictly%20rust-blueviolet.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Author](https://img.shields.io/badge/author-Mert%20Kızılırmak%20(RedRiveRR)-blue.svg)

**Directory Bruteforcer**, Rust ile yazılmış yüksek performanslı bir dizin ve dosya keşif (bruteforce) aracıdır. Web sunucularındaki gizli yolları eşzamanlı ağ istekleri yardımıyla son derece hızlı bir şekilde bulmak için tasarlanmıştır.

Kullanım kolaylığı sağlayan iki güçlü arayüz ile birlikte gelir:
1. **Komut Satırı Arayüzü (CLI)**: Terminal tutkunları için, gerçek zamanlı hiyerarşik (ağaç yapısında) sonuç görüntüleme desteği sunar.
2. **Modern Web Arayüzü**: Tamamen yerel (local) olarak çalışan, şık, karanlık temalı (dark mode) ve "glassmorphism" tasarım estetiğine sahip gelişmiş bir kontrol panelidir.

## ✨ Yeni Nesil Özellikler (v0.8.0)

- **Akıllı İçerik Analizi (Smart Content)**: Bulunan yolların sadece HTTP Status koduna bakmak yerine arka planda içeriğini (`<title>`) ve boyutunu (`Content-Length`) çıkararak olası false-positive durumlarını filtrelemenize yardımcı olur.
- **Yeniden Yinelemeli (Recursive) Tarama**: Bulunan dizinlerin altına otomatik olarak inme yeteneği (örneğin `/admin` -> `/admin/config.php`).
- **Teknoloji Parmak İzi (Fingerprinting)**: Hedefteki yazılım yığınını (WordPress, Node.js, PHP, Docker vb.) otomatik olarak teşhis eder ve görsel rozetlerle gösterir.
- **Soft-404 Heuristic Filtresi**: Sunucunun rastgele yollara `200 OK` döndürdüğü (false-positive) durumları "Heuristic Calibration" ile anında tespit eder ve temiz veri sunar.
- **Otomatik WAF Tespiti**: Sistem, hedefin bir WAF (Web Application Firewall) arkasında olduğunu veya Rate-Limiting uygulandığını sezdiğinde otomatik uyarı mekanizmasını tetikler.
- **Akıllı Mod (Smart Mode)**: Dosya seçmekle uğraşmayın. Sistem, en yaygın kullanılan dizin ve dosyaları (admin, backup, .env vb.) otomatik olarak tarar.
- **Adaptif İş Parçacığı (Adaptive Threading)**: Hedef sunucunun hızına göre otomatik vites artırır. Sunucu yavaşladığında veya hata verdiğinde sistem hızı düşürür, rahatladığında ise maksimum hıza çıkar.
- **Işık Hızında**: Maksimum istek hızı için tamamen asenkron Rust (`tokio` ve `reqwest`) altyapısı üzerine inşa edilmiştir.
- **Gerçek Zamanlı Veri Akışı (SSE)**: Web arayüzü, arka planda tarama sürerken yeni keşfedilen adresleri ve güncel tarama hızını anında ekrana yansıtır.
- **HTML Crawler**: Sayfa içerisindeki tüm dahili bağlantıları anlık olarak ayrıştırır ve otomatik olarak tarama kuyruğuna ekler. Dizin bruteforce ile ulaşılamayan yolları dinamik olarak keşfeder.
- **Gelişmiş Raporlama**: Analiz sonuçlarını temiz bir JSON veya CSV formatında dışa aktarma (export) desteği.

## Kurulum

Sisteminizde [Rust ve Cargo'nun](https://rustup.rs/) yüklü olduğundan emin olun.

Depoyu klonlayıp hemen derleyebilirsiniz:

```bash
git clone <repository_url>
cd ISU-SecOps-Engine
cargo build --release
```

Derleme bittikten sonra çalıştırılabilir programınız `target/release/dirbrute.exe` dizininde hazır olacaktır.

## Kullanım

### 🌐 Web Arayüzü (Önerilen)

Entegre web arayüzünü başlatmak için `web` komutunu kullanmanız yeterlidir.

```bash
cargo run --release -- web
```

- Özel bir port belirlemek isterseniz: `cargo run --release -- web --port 3000`

### 💻 Komut Satırı Arayüzü (CLI)

**Temel Akıllı Tarama (Önerilen):**
Sistemin wordlist'i ve hızı otomatik ayarlaması için:
```bash
cargo run --release -- pentest dirbrute "http://example.com" --auto-wordlist --auto-threads --crawler
```

**Sonuçları Dışa Aktarma:**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --output reports/results.json
```

#### CLI Parametre Tablosu:
| Parametre | Kısa Gösterim | Açıklama |
| :--- | :--- | :--- |
| `url` | - | Taranacak olan hedef URL (Zorunlu) |
| `--wordlist` | `-w` | Wordlist dosyasının konumu (Auto mod kapalıysa zorunlu) |
| `--auto-wordlist` | - | Akıllı (statik) kelime listesi modunu açar |
| `--auto-threads` | - | Adaptif işlem hızı modunu açar |
| `--threads` | `-t` | Eşzamanlı istek sayısı (Varsayılan: 10) |
| `--output` | `-o` | Sonuçların kaydedileceği dosya yolu |
| `--format` | `-f` | Çıktı formatı: `json`, `csv` |
| `--header` | `-H` | İsteğe eklenecek özel HTTP Başlığı |
| `--cookie` | `-c` | İsteğe eklenecek özel HTTP Çerezi |
| `--crawler` | `-C` | HTML Crawler motorunu aktifleştirir |

## 🛡️ Code Quality & Testing

Proje, yazılım mühendisliği standartlarına uygun olarak geliştirilmektedir:
- **Strict Linting**: `cargo clippy` ile statik kod analizi ve en iyi pratikler zorunlu tutulur.
- **Auto-Formatting**: `cargo fmt` ile kod tabanı standart Rust stilindedir.
- **CI/CD**: Her push işleminde GitHub Actions üzerinde `cargo test` ve `cargo build` otomatik olarak çalıştırılır.
- **Just/Makefile**: Sık kullanılan komutlar bir `Makefile` ile standartlaştırılmıştır (`make build`, `make test`, `make lint`).

## Dokümantasyon

Daha fazla detay için `docs/` klasöründeki Türkçe rehberlere göz atabilirsiniz:
- [Mimari Detaylar](docs/Architecture.md)
- [Test Prosedürleri](docs/Testing.md)
- [Değişim Günlüğü](docs/Changelog.md)
- [Güvenlik Politikası](SECURITY.md)

## Lisans

Bu proje [MIT Lisansı](LICENSE) ile lisanslanmıştır. (c) 2026 Mert Kızılırmak (RedRiveRR)

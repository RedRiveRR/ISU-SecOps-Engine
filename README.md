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
1. **Komut Satırı Arayüzü (CLI)**: Terminal tutkunları için gerçek zamanlı hiyerarşik (ağaç yapısında) sonuç görüntüleme desteği sunar.
2. **Modern Web Arayüzü**: Tamamen yerel (local) olarak çalışan; şık, karanlık temalı (dark mode) ve "glassmorphism" tasarım estetiğine sahip gelişmiş bir kontrol panelidir.

## ✨ Yeni Nesil Özellikler (v1.0.0)

- **Akıllı İçerik Analizi (Smart Content)**: Bulunan yolların sadece HTTP durum koduna bakmak yerine arka planda içeriğini (`<title>`) ve boyutunu (`Content-Length`) çıkararak olası yanlış pozitif (false-positive) durumlarını filtrelemenize yardımcı olur.
- **Özyinelemeli (Recursive) Tarama**: Bulunan dizinlerin altına otomatik olarak inme yeteneği (örneğin `/admin` -> `/admin/config.php`).
- **Teknoloji Parmak İzi (Fingerprinting)**: Hedefteki yazılım yığınını (WordPress, Laravel, Django, Docker vb.) otomatik olarak teşhis eder ve görsel rozetlerle gösterir.
- **Deep Stealth Mode (Derin Gizlilik)**: Güçlü WAF (Cloudflare vb.) tespit sistemlerini atlatabilmek için Contextual Blending (araya zararsız Decoy/Yanıltıcı istekler serpiştirme) ve otonom soğuma (Cooldown) refleksleri barındırır.
- **Rotating Proxy Pool (Dönen Proxy)**: İsteklerinizi birbirinden bağımsız yüzlerce IP üzerinden geçiren rastgele proxy rotasyon motoruna sahiptir. Çoklu Proxy havuzu tanımlanabilir (HTTP/SOCKS5).
- **Soft-404 Sezgisel (Heuristic) Filtresi**: Sunucunun rastgele yollara `200 OK` döndürdüğü durumları "Heuristic Calibration" ile anında tespit eder ve temiz veri sunar.
- **Otomatik WAF Tespiti**: Sistem, hedefin bir WAF (Web Application Firewall) arkasında olduğunu veya hız sınırlaması (Rate-Limiting) uygulandığını sezdiğinde otomatik uyarı mekanizmasını tetikler.
- **Akıllı Mod (Smart Mode)**: Dosya seçmekle uğraşmayın. Sistem, en yaygın kullanılan dizin ve dosyaları (admin, backup, .env vb.) otomatik olarak tarar.
- **Uyarlanabilir İş Parçacığı (Adaptive Threading)**: Hedef sunucunun hızına göre otomatik vites artırır. Sunucu yavaşladığında veya hata verdiğinde sistem hızı düşürür, rahatladığında ise maksimum hıza çıkar.
- **Işık Hızında**: Maksimum istek hızı için tamamen asenkron Rust (`tokio` ve `reqwest`) altyapısı üzerine inşa edilmiştir.
- **Gerçek Zamanlı Veri Akışı (SSE)**: Web arayüzü, arka planda tarama sürerken yeni keşfedilen adresleri ve güncel tarama hızını anında ekrana yansıtır.
- **HTML Crawler**: Sayfa içerisindeki tüm dahili bağlantıları anlık olarak ayrıştırır ve otomatik olarak tarama kuyruğuna ekler. Dizin bruteforce ile ulaşılamayan yolları dinamik olarak keşfeder.
- **Gelişmiş Raporlama**: Analiz sonuçlarını temiz bir JSON veya CSV formatında dışa aktarma (export) desteği mevcuttur.

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
Sistemin kelime listesini ve hızı otomatik ayarlaması için:
```bash
cargo run --release -- pentest dirbrute "http://example.com" --auto-wordlist --auto-threads --crawler
```

**Hayalet Tarama (Stealth & Proxy Pool):**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --stealth --proxy "http://127.0.0.1:8080,socks5://192.168.1.5:1080"
```

**Sonuçları Dışa Aktarma:**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --output reports/results.json
```

#### CLI Parametre Tablosu:
| Parametre | Kısa Gösterim | Açıklama |
| :--- | :--- | :--- |
| `url` | - | Taranacak olan hedef URL (Zorunlu) |
| `--wordlist` | `-w` | Kelime listesi dosyasının konumu (Auto mod kapalıysa zorunlu) |
| `--auto-wordlist` | - | Akıllı (statik) kelime listesi modunu açar |
| `--auto-threads` | - | Uyarlanabilir işlem hızı modunu açar |
| `--threads` | `-t` | Eşzamanlı istek sayısı (Varsayılan: 10) |
| `--output` | `-o` | Sonuçların kaydedileceği dosya yolu |
| `--format` | `-f` | Çıktı formatı: `json`, `csv` |
| `--header` | `-H` | İsteğe eklenecek özel HTTP başlığı |
| `--cookie` | `-c` | İsteğe eklenecek özel HTTP çerezi |
| `--crawler` | `-C` | HTML Crawler motorunu aktifleştirir |
| `--stealth` | `-s` | WAF Evasion & Deep Stealth modunu etkinleştirir |
| `--proxy` | `-p` | HTTP(S)/Socks5 Proxy adresi (veya virgülle ayrılmış havuz) |

## 🛡️ Kod Kalitesi ve Testler

Proje, yazılım mühendisliği standartlarına uygun olarak geliştirilmektedir:
- **Strict Linting**: `cargo clippy` ile statik kod analizi ve en iyi pratikler zorunlu tutulur.
- **Otomatik Formatlama**: `cargo fmt` ile kod tabanı standart Rust stilindedir.
- **CI/CD**: Her push işleminde GitHub Actions üzerinde `cargo test` ve `cargo build` otomatik olarak çalıştırılır.
- **Geliştirici Araçları**: Proje, `Justfile` ve `.vscode` yapılandırmaları ile tam otomatize edilmiştir.

## 🛠️ Geliştirici Rehberi

Projenin geliştirilmesine katkıda bulunmak veya yerel ortamda özelleştirmek için aşağıdaki araçlar hazırlandı:

### 1. VS Code Entegrasyonu
Proje, `.vscode` klasörü altında tam kapsamlı yapılandırmalarla gelir:
- **Otomasyon (tasks.json)**: `Build`, `Lint` ve `Test` görevleri tanımlıdır (Ctrl+Shift+B).
- **Hata Ayıklama (launch.json)**: LLDB ile hem ana programı hem de testleri adım adım debug edebilirsiniz (F5).
- **Önerilen Eklentiler (extensions.json)**: Açılışta `rust-analyzer`, `CodeLLDB` ve `Even Better TOML` gibi kritik eklentiler önerilir.

### 2. Komut Çalıştırıcı (Just / Makefile)
Sık kullanılan operasyonlar için `Justfile` (veya `Makefile`) kullanabilirsiniz:
- `just fmt`: Tüm kodu ve TOML dosyalarını formatlar.
- `just lint`: Clippy ile katı kuralları denetler.
- `just audit`: `cargo-deny` ile güvenlik ve lisans denetimi yapar.
- `just ci`: Tüm dokunulmazlık testlerini (`fmt`, `lint`, `audit`, `test`) tek seferde koşturur.

### 3. Güvenlik Denetimi
Proje, `deny.toml` yapılandırması ile tedarik zinciri güvenliğini önceler:
- Zafiyet barındıran paketler (`advisories`) otomatik olarak engellenir.
- Uyumsuz lisanslar (Copyleft vb.) reddedilir.

## Dokümantasyon

Daha fazla detay için `docs/` klasöründeki Türkçe rehberlere göz atabilirsiniz:
- [Mimari Detaylar](docs/Architecture.md)
- [Test Prosedürleri](docs/Testing.md)
- [Değişim Günlüğü](docs/Changelog.md)
- [Güvenlik Politikası](SECURITY.md)

## Lisans

Bu proje [MIT Lisansı](LICENSE) ile lisanslanmıştır. © 2026 Mert Kızılırmak (RedRiveRR)

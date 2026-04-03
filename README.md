# ISU SecOps Engine (v0.3.0)

ISU SecOps Engine, Rust ile yazılmış yüksek performanslı bir dizin ve dosya keşif (bruteforce) aracıdır. Web sunucularındaki gizli yolları eşzamanlı ağ istekleri yardımıyla son derece hızlı bir şekilde bulmak için tasarlanmıştır.

Kullanım kolaylığı sağlayan iki güçlü arayüz ile birlikte gelir:
1. **Komut Satırı Arayüzü (CLI)**: Terminal tutkunları için, gerçek zamanlı hiyararşik (ağaç yapısında) sonuç görüntüleme desteği sunar.
2. **Modern Web Arayüzü**: Tamamen yerel (local) olarak çalışan, şık, karanlık temalı (dark mode) ve "glassmorphism" tasarım estetiğine sahip gelişmiş bir kontrol panelidir.

## ✨ Yeni Nesil Özellikler (v0.3.0)

- **Akıllı Mod (Smart Mode)**: Dosya seçmekle uğraşmayın. Sistem, en yaygın kullanılan dizin ve dosyaları (admin, backup, .env vb.) otomatik olarak tarar.
- **Adaptif İş Parçacığı (Adaptive Threading)**: Hedef sunucunun hızına göre otomatik vites artırır. Sunucu yavaşladığında veya hata verdiğinde sistem hızı düşürür, rahatladığında ise maksimum hıza çıkar.
- **Işık Hızında**: Maksimum istek hızı için tamamen asenkron Rust (`tokio` ve `reqwest`) altyapısı üzerine inşa edilmiştir.
- **Çift Arayüz (Dual Interface)**: Node.js gibi harici hiçbir sistem gereksinimi duymadan her iki arayüzü de kullanabilirsiniz.
- **Gerçek Zamanlı Veri Akışı (SSE)**: Web arayüzü, arka planda tarama sürerken yeni keşfedilen adresleri ve güncel tarama hızını anında ekrana yansıtır.

## Kurulum

Sisteminizde [Rust ve Cargo'nun](https://rustup.rs/) yüklü olduğundan emin olun.

Depoyu klonlayıp hemen derleyebilirsiniz:

```bash
git clone <repository_url>
cd ISU-SecOps-Engine
cargo build --release
```

Derleme bittikten sonra çalıştırılabilir programınız `target/release/secops.exe` dizininde hazır olacaktır.

## Kullanım

### 🌐 Web Arayüzü (Önerilen)

Entegre web arayüzünü başlatmak için `web` komutunu kullanmanız yeterlidir.

```bash
cargo run --release -- web
```

- Özel bir port belirlemek isterseniz: `cargo run --release -- web --port 3000`
- Arayüzden **Smart Mode** togglle'larını açarak sistemin hızı ve yolları kendi seçmesini sağlayabilirsiniz.

### 💻 Komut Satırı Arayüzü (CLI)

**Temel Akıllı Tarama (Önerilen):**
Sistemin wordlist'i ve hızı otomatik ayarlaması için:
```bash
cargo run --release -- pentest dirbrute "http://example.com" --auto-wordlist --auto-threads
```

**Gelişmiş Manuel Kullanım:**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --wordlist common.txt --threads 20 --header "Authorization: Bearer token" --cookie "session=123"
```

#### CLI Parametre Tablosu:
| Parametre | Kısa Gösterim | Açıklama |
| :--- | :--- | :--- |
| `url` | - | Taranacak olan hedef URL (Zorunlu) |
| `--wordlist` | `-w` | Wordlist dosyasının konumu (Auto mod kapalıysa zorunlu) |
| `--auto-wordlist` | - | Akıllı (statik) kelime listesi modunu açar |
| `--auto-threads` | - | Adaptif işlem hızı modunu açar |
| `--threads` | `-t` | Eşzamanlı istek sayısı (Varsayılan: 10) |
| `--header` | `-H` | İsteğe eklenecek özel HTTP Başlığı |
| `--cookie` | `-c` | İsteğe eklenecek özel HTTP Çerezi |

## Dokümantasyon

Daha fazla detay için `docs/` klasöründeki Türkçe rehberlere göz atabilirsiniz:
- [Mimari Detaylar](docs/Architecture.md)
- [Test Prosedürleri](docs/Testing.md)
- [Değişim Günlüğü](docs/Changelog.md)

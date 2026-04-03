# ISU SecOps Engine

ISU SecOps Engine, Rust ile yazılmış yüksek performanslı bir dizin ve dosya keşif (bruteforce) aracıdır. Web sunucularındaki gizli yolları eşzamanlı ağ istekleri yardımıyla son derece hızlı bir şekilde bulmak için tasarlanmıştır.

Kullanım kolaylığı sağlayan iki güçlü arayüz ile birlikte gelir:
1. **Komut Satırı Arayüzü (CLI)**: Terminal tutkunları için, gerçek zamanlı hiyerarşik (ağaç yapısında) sonuç görüntüleme desteği sunar.
2. **Modern Web Arayüzü**: Tamamen yerel (local) olarak çalışan, şık, karanlık temalı (dark mode) ve "glassmorphism" tasarım estetiğine sahip gelişmiş bir kontrol panelidir.

## Özellikler

- **Işık Hızında**: Maksimum istek hızı için tamamen asenkron Rust (`tokio` ve `reqwest`) altyapısı üzerine inşa edilmiştir.
- **Çift Arayüz (Dual Interface)**: Node.js gibi harici hiçbir sistem gereksinimi duymadan her iki arayüzü de kullanabilirsiniz.
- **Gerçek Zamanlı Veri Akışı (SSE)**: Web arayüzü, arka planda tarama sürerken yeni keşfedilen adresleri Server Sent Events aracılığıyla beklemeden anında ekrana yansıtır.
- **Detaylı Özelleştirme**: Özel HTTP Başlıkları (Headers), Çerezler (Cookies) ve Thread (iş parçacığı) sayısını belirleyerek taramalarınızı tam kontrolde tutabilirsiniz.
- **Durum Kodu (Status Code) Vurgulama**: Önemli durum kodlarını (200, 301, 302, 401, 403, 500 vb.) hem konsol çıktısında hem de web arayüzünde otomatik ayırt edip renklendirir.

## Kurulum

Sisteminizde [Rust ve Cargo'nun](https://rustup.rs/) yüklü olduğundan emin olun.

Depoyu klonlayıp hemen derleyebilirsiniz:

```bash
git clone <repository_url>
cd ISU-SecOps-Engine
cargo build --release
```

Derleme bittikten sonra çalıştırılabilir programınız `target/release/secops.exe` (Linux/macOS için `secops`) dizininde hazır olacaktır.

## Kullanım

### 🌐 Web Arayüzü (Önerilen)

Entegre web arayüzünü başlatmak için `web` komutunu kullanmanız yeterlidir.

```bash
cargo run --release -- web
```

- Özel bir port belirlemek isterseniz (Varsayılan 8080'dir): `cargo run --release -- web --port 3000`
- Başlatmanın ardından tarayıcınızdan `http://127.0.0.1:8080` adresine gidebilirsiniz.

### 💻 Komut Satırı Arayüzü (CLI)

Arayüz yerine Terminal sistemini tercih ederseniz, `pentest dirbrute` komutuyla doğrudan tarama yapabilirsiniz.

**Temel Kullanım:**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --wordlist common.txt
```

**Gelişmiş Kullanım:**
```bash
cargo run --release -- pentest dirbrute "http://example.com" --wordlist common.txt --threads 20 --header "Authorization: Bearer token" --cookie "session=123"
```

#### CLI Parametre Tablosu:
| Parametre | Kısa Gösterim | Açıklama |
| :--- | :--- | :--- |
| `url` | - | Taranacak olan hedef URL (Zorunlu) |
| `--wordlist` | `-w` | Dizin/dosya yollarını barındıran kelime listesinin konumu (Zorunlu) |
| `--threads` | `-t` | Eşzamanlı atılacak istek (thread) sayısı (Varsayılan: 10) |
| `--header` | `-H` | İsteğe eklenecek özel HTTP Başlığı (Örn. "X-Custom: Value") |
| `--cookie` | `-c` | İsteğe eklenecek özel HTTP Çerezi (Cookie) |

# Temel Mimari (Core Architecture)

Directory Bruteforcer, Rust ile geliştirilmiş, yüksek performanslı ve asenkron (eşzamanlı) çalışan bir güvenlik aracıdır.

## Modül Detayları

### 1. `main.rs` (Giriş Noktası)
`clap` kütüphanesini kullanarak komut satırı argümanlarını ayrıştırır ve ilgili modüle yönlendirme yapar:
- `Command::Pentest`: `scanner` (tarayıcı) modülüne yönlendirir.
- `Command::Web`: `web` modülüne yönlendirir.

### 2. `cli.rs` (Komut Satırı Arayüz Tanımı)
CLI yapısını, alt komutları ve bayrakları (flags) tanımlar. `clap` kütüphanesinin makrolarını kullanarak kullanıcı dostu bir arayüz sağlar.

### 3. `scanner.rs` (Çekirdek Tarama Motoru)
Projenin kalbi olan bu modül şu özelliklere sahiptir:
- **Asenkron Çalışma**: Bloklamayan I/O için `tokio`, paralel istek yönetimi için `futures` kullanır.
- **Olay Odaklı İletişim (Event-Driven)**: Çekirdek motor, sonuçları doğrudan ekrana basmak yerine `tokio::sync::mpsc` kanalları üzerinden `ScanEvent` mesajları yayınlar. Bu sayede sonuçlar aynı anda hem CLI hem de Web arayüzü tarafından canlı olarak işlenebilir.
- **Deep Stealth Mode (Derin Gizlilik)**: WAF ve IPS sistemlerini atlatmak için otonom *Contextual Blending* (decoy requests) ve *Cooldown* (soğuma) mekanizmalarını yönetir.
- **Rotating Proxy Pool**: Birden fazla proxy sunucusunu `Arc<Vec<reqwest::Client>>` yapısında yöneterek her istekte rastgele IP değişimi (rotation) yapar.
- **URL Normalizasyonu**: Eksik protokolleri (http/https) tespit eder ve varsayılan olarak `https://` ekleyerek hataları önler.

### 4. `web.rs` (Web Arayüz Sunucusu)
`axum` kütüphanesi üzerine inşa edilmiş entegre bir sunucudur:
- **Gömülü Varlıklar (Embedded Assets)**: Tüm frontend dosyaları `include_str!` ile ikili dosyaya (binary) gömülür, böylece dış bağımlılık olmadan çalışır.
- **Server-Sent Events (SSE)**: Tarayıcıya, tarama motorundan gelen `ScanEvent` mesajlarını anlık olarak akış (stream) şeklinde iletir.
- **REST API**: Taramaları başlatmak ve izlemek için basit JSON uç noktaları sunar.

### 5. `ui/index.html` (Frontend)
Modern ve tek sayfalık bir frontend yapısı:
- **Vanilla JS**: SSE bağlantısını yönetir ve UI durumunu günceller.
- **Modern CSS**: Karanlık mod ve gelişmiş cam efekti (glassmorphism) tasarımıyla üst segment bir deneyim sunar.

## Veri Akışı

1. Kullanıcı taramayı başlatır (CLI veya Web üzerinden).
2. `scanner::run_dirbrute_core` fonksiyonu ayrı bir asenkron görev (task) olarak başlatılır.
3. Motor, kelime listesini (wordlist) okur ve eşzamanlı HTTP isteklerini gönderir.
4. Önemli bir sonuç (örneğin 200 OK) bulunduğunda, kanal üzerinden bir `ScanEvent::Found` mesajı gönderilir.
5. Alıcı (CLI yazıcısı veya Web SSE işleyicisi) bu mesajı alır ve kullanıcı arayüzünü anında günceller.

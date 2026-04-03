# 🏗️ Core Architecture (Temel Mimari)

`dirbrute`, eşzamanlılık (concurrency) ve düşük gecikme süreli (low-latency) I/O operasyonları için Rust dilinin asenkron ekosistemi üzerine inşa edilmiştir.

## 🧱 Modüler Yapı

### 1. `main.rs` & `cli.rs` (Execution Entry & Configuration)
Uygulama, `clap` kütüphanesi ile yapılandırılmış tip-güvenli bir komut satırı arayüzü sunar. Komutlar iki ana iş akışına ayrılır:
- **Pentest Engine**: `scanner` modülünü asenkron bir `tokio::task` olarak başlatır.
- **Web Interface**: `axum` tabanlı bir HTTP sunucusu ayağa kaldırarak grafik arayüz desteği sağlar.

### 2. `scanner.rs` (Search Engine Core)
Sistemin çekirdeğini oluşturan asenkron tarama motoru:
- **Non-blocking I/O**: `tokio` ve `reqwest` ile bloklamayan ağ istekleri yönetilir.
- **Event-Driven Messaging**: Motor, durum güncellemelerini ve bulguları `tokio::sync::mpsc` kanalları üzerinden asenkron olarak yayınlar. Bu mimari, CLI ve Web arayüzünün aynı anda canlı veri tüketmesine olanak tanır.
- **WAF Evasion Logic**: `Deep Stealth` modunda, trafik örüntülerini randomize eden otonom soğuma ve decoy istek algoritmaları bu seviyede işletilir.
- **Client Pooling**: Proxy rotasyonu için `Arc<Vec<Client>>` yapısı kullanılarak thread-safe bir bağlantı havuzu yönetilir.

### 3. `web.rs` & `ui/` (Communication Layer)
- **Axum Framework**: RESTful uç noktalar ve statik dosya sunumu için optimize edilmiştir.
- **Streaming Result Delivery**: Tarayıcı ile motor arasındaki bağlantı **Server-Sent Events (SSE)** ile kurulur. Bu sayede her bulgu anında kullanıcı ekranına yansır.
- **Frontend Assets**: Tüm HTML/CSS/JS varlıkları derleme aşamasında binary içerisine gömülür (embedded), bu da uygulamayı tek bir bağımsız dosya haline getirir.

## 🔄 Veri Akışı (Data Flow)

1. **Initialization**: Kullanıcı girdileri doğrulanır ve `Wordlist` nesnesi belleğe yüklenir.
2. **Channel Setup**: Motor ile arayüzler arasında mesajlaşma kanalları kurulur.
3. **Execution**: Paralel işçiler (workers) kelime listesini işlerken, WAF sezgileme motoru sürekli geri bildirim toplar.
4. **Event Emission**: Bulunan her yol için bir `ScanEvent` oluşturulur ve kanala asenkron olarak basılır.
5. **Consumption**: CLI yazıcısı veya Web SSE işleyicisi gelen mesajları terminale veya tarayıcıya asenkron olarak yansıtır.

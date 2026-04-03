# 🛸 dirbrute v1.1.0
## *High-Performance Stealth Discovery & Security Assessment Engine*

<div align="center">
  <img src="https://img.shields.io/badge/VERSION-1.1.0-blue?style=for-the-badge&logo=github" />
  <img src="https://img.shields.io/badge/RUST-1.75%2B-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/LICENSE-MIT-green?style=for-the-badge" />
  <img src="https://img.shields.io/badge/PLATFORM-WINDOWS%20%7C%20LINUX%20%7C%20MACOS-lightgrey?style=for-the-badge" />
  <br>
  <img src="https://img.shields.io/badge/MAINTENANCE-ACTIVE-brightgreen?style=flat-square" />
  <img src="https://img.shields.io/badge/PRs-WELCOME-blueviolet?style=flat-square" />
  <img src="https://img.shields.io/badge/SECURE-WAF_EVASION-red?style=flat-square" />
  <img src="https://img.shields.io/badge/INTERFACE-WEB_%7C_CLI-blue?style=flat-square" />
</div>

---

### 📖 Giriş

**dirbrute**, modern sızma testleri ve güvenlik denetimleri için Rust ile geliştirilmiş, ultra hızlı ve düşük izli (low-footprint) bir keşif motorudur. Standart tarayıcıların ötesine geçerek **Deep Stealth** ve **Rotating Proxy** teknolojileriyle en katı güvenlik duvarlarını (WAF) dahi profesyonel bir hassasiyetle aşmanıza olanak tanır.

---

### 💎 Temel Modüller ve Özellikler

#### 🥷 1. Deep Stealth & WAF Evasion (v1.1.0)
Modern firewall sistemleri (Cloudflare, Akamai, AWS Shield) basit bruteforce denemelerini anında engeller. `dirbrute` bu engelleri aşmak için şu metodları kullanır:
- **Contextual Blending**: Tarama trafiği arasına otomatik olarak `/robots.txt`, `/favicon.ico`, `/assets/logo.png` gibi zararsız "decoy" istekler serpiştirerek trafiği gerçek kullanıcı gibi gösterir.
- **Autonomous Cooldown**: Sistem bloklandığını (403/429) anladığında tüm iş parçacıklarını 60 saniye boyunca otonom olarak uyutur ve "tehdit algısını" sıfırlar.
- **Jitter & UA Rotation**: Her istek arasına mikro-gecikmeler ekler ve binlerce gerçek tarayıcı "User-Agent" bilgisi arasında rotasyon yapar.

#### 🔄 2. Rotating Proxy Pool (v1.1.0)
Tek bir IP üzerinden yapılan binlerce istek, ne kadar gizli olsa da yakalanır. 
- **Proxy Rotation**: Virgülle ayrılmış yüzlerce HTTP/SOCKS5 proxy adresi tanımlayabilirsiniz.
- **Random Picking**: Her bir işçi (worker), her yeni istek için havuzdan rastgele bir proxy seçerek çıkış IP'sini sürekli değiştirir.

#### 🛰️ 3. Dynamic HTML Crawler
Bruteforce ile ulaşılamayan JS dosyaları, CSS'ler veya derin sayfalar içindeki URL'leri `nipper` motoruyla gerçek zamanlı olarak ayrıştırır. Bulunan tüm dahili bağlantıları otomatik olarak tarama kuyruğuna ekler.

#### 🧠 4. Smart Content Discovery
- **Technology Fingerprinting**: Hedefin WordPress, Laravel, Docker, Django veya GraphQL olduğunu anında tespit eder ve ona göre tarama önceliği belirler.
- **Heuristic 404 Filter**: "Sahte 200" (Soft-404) dönen sitelerde baseline analizi yaparak tüm yanlış pozitifleri eler.
- **Adaptive Threading**: Sunucu yanıt sürelerini ölçer; sunucu hızlıysa hızlanır, gecikme artarsa otomatik vites küçültür.

---

### 🚀 Hızlı Başlangıç

#### 🛠️ Kurulum
```bash
git clone https://github.com/RedRiveRR/ISU-SecOps-Engine
cd ISU-SecOps-Engine
cargo build --release
```

#### 🌐 Web Kontrol Paneli (Web UI)
Modern, cam efektli (Glassmorphism) ve gerçek zamanlı karanlık mod paneli:
```bash
cargo run --release -- web --port 8080
```

#### 💻 CLI - Gelişmiş Kullanım
```bash
# Akıllı ve Hayalet Mod kombinasyonu
cargo run --release -- pentest dirbrute "http://target.com" --stealth --crawler --auto-wordlist --proxy "http://p1:8080,socks5://p2:1080"
```

---

### 📊 CLI Parametre Referansı

| Parametre | Kısa | Açıklama |
| :--- | :--- | :--- |
| `url` | - | Hedef URL (Zorunlu) |
| `--wordlist` | `-w` | Özel kelime listesi yolu |
| `--auto-wordlist` | - | Akıllı (dahili) kelime listesini açar |
| `--auto-threads` | - | Otomatik hız ayarlama (Adaptive) |
| `--threads` | `-t` | Maksimum eşzamanlı istek (Varsayılan: 10) |
| `--stealth` | `-s` | **Deep Stealth** modunu aktif eder |
| `--proxy` | `-p` | Virgülle ayrılmış proxy havuzu |
| `--crawler` | `-C` | HTML Crawler motorunu açar |
| `--output` | `-o` | Rapor yolu (JSON/CSV) |

---

### 🏗️ Teknoloji Stack & Geliştirici Ekosistemi

<div align="center">
  <img src="https://img.shields.io/badge/Tokio-Runtime-000000?style=flat&logo=rust" />
  <img src="https://img.shields.io/badge/Axum-API-blue?style=flat&logo=rust" />
  <img src="https://img.shields.io/badge/Reqwest-HTTP-orange?style=flat&logo=rust" />
  <img src="https://img.shields.io/badge/Serde-Serialization-yellow?style=flat&logo=rust" />
</div>

Proje geliştiriciler için tam otomatize edilmiştir:
- **`Justfile`**: `just ci` komutuyla fmt, lint, audit ve test adımlarını tek seferde yapabilirsiniz.
- **`.vscode`**: Hata ayıklama (LLDB) ve görev otomasyonları (Tasks) yapılandırmalarıyla hazır olarak gelir.
- **Security**: `cargo-deny` ile tedarik zinciri (supply chain) ve zafiyet analizi her derlemede zorunludur.

---

### 📜 Lisans & Sorumluluk Reddİ

Bu proje **MIT Lisansı** ile lisanslanmıştır. Araç yalnızca yasal penetrasyon testleri ve eğitim amaçlı tasarlanmıştır. İzinsiz kullanımda tüm sorumluluk kullanıcıya aittir.

**Geliştiren:** [Mert Kızılırmak (RedRiveRR)](https://github.com/RedRiveRR)

---
<div align="center">
  <sub>Built with ❤️ and 🦀 by the ISU SecOps Team</sub>
</div>

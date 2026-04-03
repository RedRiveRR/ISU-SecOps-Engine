# Test Prosedürü (Testing)

Directory Bruteforcer hem Komut Satırı Arayüzü (CLI) hem de Web Arayüzü modlarında test edilebilir.

## CLI Modu Testi

Merkezi tarama motorunu doğrulamak için:
1. Hedef web sunucusunun çalıştığından emin olun.
2. `pentest dirbrute` alt komutunu çalıştırın.
3. Sonuçların, renkli durum kodlarıyla birlikte hiyerarşik (ağaç) yapıda doğru yazılıp yazılmadığını kontrol edin.

**Çalıştırma:**
```bash
cargo run --release -- pentest dirbrute "http://localhost:8000" --wordlist wordlists/common.txt
```

### Doğrulama Listesi:
- [ ] Tarama başarıyla başlar.
- [ ] Kelime listesi (wordlist) doğru yüklenir.
- [ ] Bulunan yollar ağaç yapısında görüntülenir.
- [ ] Durum kodları uygun renklerde vurgulanır (200 için Yeşil, 301+ için Sarı, 400+ için Kırmızı).
- [ ] `--stealth` bayrağı ile decoy istekler (`[🥷 STEALTH]`) terminale basılır.
- [ ] `--proxy` parametresi ile istekler belirtilen vekil sunucu üzerinden akar.

## Web UI Modu Testi

Entegre web sunucusunu doğrulamak için:
1. `web` alt komutunu çalıştırın.
2. Tarayıcınızda `http://127.0.0.1:8080` adresine gidin.
3. Test yapılandırmasını girin, "Deep Stealth Mode" seçeneğini aktif edin ve "Launch Scan" butonuna tıklayın.

**Çalıştırma:**
```bash
cargo run --release -- web --port 8080
```

### Doğrulama Listesi:
- [ ] Sunucu başlar ve belirtilen portu dinler.
- [ ] Web arayüzü erişilebilirdir ve doğru şekilde render edilir (Karanlık mod, animasyonlar).
- [ ] "Deep Stealth Mode" aktifken Live Logs sekmesinde mor renkli hayalet logları görünür.
- [ ] "Proxy Engine" alanına girilen virgüllü listeler (rootation) hata vermeden işlenir.
- [ ] Sonuçlar bulundukça gerçek zamanlı (SSE) olarak görüntülenir.
### ✅ Kabul Kriterleri (Checklist):
- [ ] Web sunucusu belirtilen portu "non-blocking" modda dinlemeye başlar.
- [ ] Arayüzde "Karanlık Mod" ve "Glassmorphism" efektleri hatasız render edilir.
- [ ] SSE akışı sayesinde bulgular sayfayı yenilemeden (real-time) listeye eklenir.
- [ ] "Proxy Engine" alanına girilen virgüllü proxy listesi hata vermeden parse edilir.
- [ ] Tarama sonunda "Tamamlandı" statüsü ve toplam sonuç sayısı senkronize görünür.

## 🤖 Otomatik Testler & Linting

Kod kalitesini ve temel fonksiyonları korumak için Rust araç setini kullanın:

- **Unit Tests**: `cargo test` komutuyla çekirdek fonksiyonların regresyon testlerini yapın.
- **Static Analysis**: `just lint` (cargo clippy) ile katı kod standartlarını denetleyin.
- **Security Audit**: `just audit` (cargo deny) ile bağımlılık zafiyetlerini kontrol edin.

---
*(Not: Yeni özellikler eklendikçe bu test protokolü güncellenmelidir.)*

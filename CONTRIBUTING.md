# Panduan Berkontribusi

Terima kasih sudah tertarik untuk membantu mengembangkan project ini! 
Dokumen ini dibuat agar alur kerja kita tetap rapi dan terkoordinasi.

## Memulai Kontribusi

1. **Pilih Task:** Cek tab **Issues** atau board **Project**. Cari issue dengan label `help wanted` atau `good first issue`.
2. **Klaim Tiket:** Beri komentar di issue tersebut (contoh: *"Saya ingin ambil tiket ini"*). Tunggu maintainer me-assign issue tersebut ke akunmu agar tidak terjadi pengerjaan ganda (*duplicate work*).

---

## Alur Kerja Git (Step-by-Step)

### 1. Clone 
Clone ke komputer kalian:
```bash
git clone https://github.com/Modularity-Community/lazy.git
cd lazy

### 2. Create branch
Buat branch baru dari branch `main` dengan format `feature/<no-issue>-<nama-singkat/nama-task>-<nama-task>` atau `fix/<no-issue>-<nama-singkat/nama-task>`

### 3. Commit dan Push
Commit perubahanmu lalu push branch langsung ke repositori ini:
```bash
git commit -m "feat(#12): implementasi form login"
git push origin feature/12-bu-form-login
```

### 4. Close task atau issue (Optional)
Cantumkan kata kunci penutup tiket di deskripsi PR agar GitHub otomatis menutup issue saat PR di-merge.

Contoh format pada deskripsi PR:

<b>Closes #12 (atau Fixes #12)</b>

Ringkasan perubahan:

- Menambahkan input email dan password
- Integrasi validasi form

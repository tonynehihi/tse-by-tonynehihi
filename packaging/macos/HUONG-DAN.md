# TS Excel — Hướng dẫn cài đặt sạch & sử dụng

Bản build cá nhân của Tony, dựa trên GridCraft của storytold (MIT/Apache-2.0), đã đổi:

- Tên app là **TS Excel** (TS = Tony Suite), tên ngoài Spotlight/Finder là **TsE**, icon **X xanh lá** kiểu Excel cùng ngôn ngữ thiết kế với cả bộ app của Tony
- **Tự update trong app**: Help → Check for Updates (xem thêm mục dưới)
- **Xóa sạch link về bản gốc** trong app: không còn Discord / website / GitHub của ArtCraft ở Help, About hay menu nào nữa; menu Help chỉ còn 4 mục: About / Check for Updates / System Info / Keyboard Shortcuts
- Thư mục settings/dữ liệu **giữ nguyên tên GridCraft như bản gốc** — cài bản này không mất settings hay danh sách file gần đây

## Mới ở bản 1.1.0 — TS Excel + tự update trong app

- **Tự kiểm tra & cài bản mới**: vào **Help → Check for Updates** (hoặc mở **About TS Excel**, mục **Updates** ở dưới cùng): app tự hỏi GitHub xem có bản mới hơn không.
- Mở app mà có bản mới thì hiện pop-up kèm **What's new** (changelog) để đọc trước khi quyết định. Bấm **Later** thì lần mở sau hỏi lại.
- Có bản mới: bấm **Download Update** để tải, rồi **Install Update** — app tự build bản mới **ngay trong app** và hiện tiến trình trực tiếp (đang ở bước nào, đã compile bao nhiêu crate, đã chạy bao lâu). Bản đang dùng không bị đụng vào cho tới khi build xong.
- Build xong bấm **Finish Install**: app thoát **vài giây** để tráo bản mới vào /Applications (bản cũ được dời vào thư mục backup `~/Library/Caches/TS Excel/updates/backup` trước, tráo lỗi thì khôi phục bản cũ và mở lại), rồi tự mở lại.
- Nếu cài đặt dở dang có trục trặc: xem log ở `~/Library/Logs/TSExcel-update.log`, build log ở `~/Library/Caches/TS Excel/updates/build-<version>/build.log`.
- Bản Windows: kiểm tra bản mới trong About được, nhưng chưa tự cài — tải bản mới ở trang release (nút **Open Release Page** trong app).

## Gỡ bản cũ / cài lại sạch

**Nếu đã build bản trước:**
```bash
# Xóa bản build cũ
rm -rf target/
# Xóa app đã cài (nếu đã kéo vào Applications)
rm -rf "/Applications/TsE.app"
rm -rf "/Applications/GridCraft.app"
```

Settings nằm ở `~/Library/Application Support/GridCraft` (giữ nguyên tên như bản gốc) — **đừng xóa** trừ khi muốn làm lại từ đầu thật.

## Cài đặt (làm 1 lần)

**Bước 1** — Cài Xcode Command Line Tools (nếu chưa có):
```
xcode-select --install
```

**Bước 2** — Cài Rust (nếu chưa có):
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Xong thì **đóng Terminal, mở lại**.

## Build & chạy

**Bước 3** — Giải nén `tse-by-tonynehihi-1.1.0.zip`.

**Bước 4** — Trong Terminal:
```bash
cd <kéo folder tse-by-tonynehihi-1.1.0 vào Terminal rồi thả>
cargo build --release -p gridcraft
```
(Lần đầu 10–20 phút.)

**Bước 5** — Chạy thử:
```bash
./target/release/gridcraft
```

**Bước 6** — Đóng gói thành app:
```bash
packaging/macos/package.sh --arch aarch64
```
(Máy Intel cũ: `--arch x86_64`.)
Xong mở file `.dmg` trong `dist/release/`, kéo **TsE.app** vào Applications.

## App không mở được (macOS chặn app tự đóng gói)

App tự đóng gói không có chữ ký Apple nên macOS hay chặn — không phải app hỏng.

**Trường hợp 1** — macOS báo app "from an unidentified developer":
Chuột phải vào app → chọn **Open** → bấm **Open** trong hộp thoại hiện ra.

**Trường hợp 2** — macOS báo app "is damaged" hoặc "can't be opened":
1. Mở Terminal.
2. Gõ đúng dòng này rồi Enter:
```bash
xattr -cr "/Applications/TsE.app"
```
3. Mở app lại bình thường.

**Trường hợp 3** — app crash / tắt ngay khi mở:
Kiểm tra lại **Bước 6** đã đúng chip máy chưa — máy Apple Silicon (M1/M2/M3/M4) phải dùng `--arch aarch64`, máy Intel cũ dùng `--arch x86_64`. Đóng gói sai chip là crash luôn, phải đóng gói lại.

## Build báo lỗi

- Thiếu `xcode-select`: chạy `xcode-select --install` rồi build lại.
- Terminal không nhận lệnh `cargo`: đóng Terminal, mở lại (Rust mới thêm vào PATH sau khi cài).
- Lỗi khác: chụp màn hình lỗi gửi tao.

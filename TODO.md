請參考這份文件
@https://github.com/openai/codex/raw/refs/heads/main/.github/workflows/rust-release.yml 
幫我重新設計 並修改 .github/workflows/build_release.yml
我希望在這個action中 將所有平台的binaries打包上傳到github release Assets

唯一不同的地方是 範例中有限制tag名稱 我的部分不需要限制那麼複雜 按照原本的方式限制 v 開頭即可
並且我不需要上傳到 npm, 我只會發發佈在 Github Release

上傳時也要注意
./target/release/${{ github.event.repository.name }}*
在windows下 會有 .exe 但是linux/macos則不會有 目前作法可能會錯誤將 *.d 一起上傳 這樣不對 所以你要妥善規劃!

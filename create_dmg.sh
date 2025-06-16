
ln -sf ../Resources/assets target/release/bundle/osx/MerinoBreakout.app/Contents/MacOS/
hdiutil create -volname "MerinoBreakout" -srcfolder "target/release/bundle/osx/MerinoBreakout.app" -ov -format UDZO "MerinoBreakout.dmg"

id := "com.mika.plugins.opendeck-duo87.sdPlugin"

package: build-linux collect zip

build-linux:
    cargo build --release --target-dir target/plugin-linux

clean:
    sudo rm -rf target/

collect:
    rm -rf build
    mkdir -p build/{{id}}
    cp -r assets build/{{id}}
    cp manifest.json build/{{id}}
    cp target/plugin-linux/release/opendeck-duo87 build/{{id}}/opendeck-duo87-linux

[working-directory: "build"]
zip:
    zip -r opendeck-duo87.plugin.zip {{id}}/
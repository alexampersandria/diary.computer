# TIME START
start=$(date +%s%N)

version="$(cat VERSION)"
image_name="diary.computer"
echo "\033[0;35m🔨 BUILD\033[0m → building Docker image: \033[0;32m${image_name}\033[0m"
docker build --pull -t "${image_name}:latest" -t "${image_name}:${version}" .

# TIME END
end=$(date +%s%N)

echo "\033[0;35m🔨 BUILD\033[0m completed in \033[0;32m$(($(($end-$start))/1000000)) ms\033[0m"

#!/usr/bin/env bash

cargo generate --git https://github.com/rustwasm/wasm-pack-template <<< "$1"
mv $1 "$1-game"
mkdir -p $1

mv "$1-game" $1/game
cat <<EOT >> $1/build.sh
#!/usr/bin/env bash

(cd game; wasm-pack build --target web -d ../web/build)
EOT
chmod +x $1/build.sh

cat <<EOT >> $1/serve.sh
#!/usr/bin/env bash

python3 -m http.server -d web
EOT
chmod +x $1/serve.sh

cat <<EOT >> $1/start.sh
#!/usr/bin/env bash

./build.sh && ./serve.sh
EOT
chmod +x $1/start.sh

mkdir $1/web
cd $1/web

cat >> index.html <<EOT
<!DOCTYPE html>
<html>
	<head>
EOT
echo "		<title>$1</title>" >> index.html
cat  >> index.html <<EOT
		<meta content="text/html;charset=utf-8" http-equiv="Content-Type">
		<meta content="utf-8" http-equiv="encoding">
		<style>
			html, body, #app {
				height: 100%;
				width: 100%;
			}

			* {
				box-sizing: border-box;
				outline: 0;
				margin: 0;
			}

			#app {
				background-color: #111;
				display: flex;
        flex-direction: column;
			}

			.container {
				flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
			}
		</style>
	</head>
	<body>
		<main id="app">
			<div class="container"></div>
		</main>
		<script async type="module" src="initialize.js"></script>
	</body>
</html>
EOT

BUILD_FILE=${1//-/_}
echo "import init from './build/$BUILD_FILE.js'" >> initialize.js
cat >> initialize.js <<EOT
import { start } from './src/game.js'

// IIFE to bootstrap WASM
(async () => {
	await init()
	start()
})()
EOT

mkdir {src,build}
echo "import { greet } from '../build/$BUILD_FILE.js'" >> src/game.js
cat >> src/game.js <<EOT

export const start = () => greet()
EOT

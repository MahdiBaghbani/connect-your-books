### **Install Node.js**

[Digital Ocean guide for Ubuntu 22.04.](https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-22-04)

#### Installing Node Using the Node Version Manager

_remember to update versions based on your needs._

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh
```

```bash
source ~/.bashrc
```

```bash
nvm list-remote
```

##### install using version number or lts name.

```bash
nvm install v16.14.0
```

```bash
nvm install lts/fermium
```

```bash
nvm list
```

### **Install Tailwind CSS**

```bash
npm install -D tailwindcss
```

install plugins:

```bash
npm install -D @tailwindcss/forms 
```

run:

```bash
NODE_ENV=production npx --yes tailwindcss -c ./tailwind.config.js -o ./public/tailwind.css --minify
```

above command will automatically execute by ``trunk`` because it is
defined in ``Trunk.toml`` 

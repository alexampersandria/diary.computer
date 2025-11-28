#!/bin/bash

# USER INPUT
echo -e '\033[0;35m🪛 SETUP\033[0m → postgres \033[0;36musername\033[0m'
echo -n -e '\033[0;32m❯\033[0m '
read username
echo -e '\033[0;35m🪛 SETUP\033[0m → postgres \033[0;36mpassword\033[0m 🤫'
echo -n -e '\033[0;32m❯\033[0m '
read -s password # hide passwordcch
echo # new line after password
echo -e '\033[0;35m🪛 SETUP\033[0m → postgres \033[0;36mdatabase\033[0m'
echo -n -e '\033[0;32m❯\033[0m '
read database

# TIME START
start=$(date +%s%N)

# ACTUAL SETUP
# Check if we're on macOS or Linux and install PostgreSQL accordingly
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS setup
    echo -e '\033[0;35m🪛 SETUP\033[0m → Installing PostgreSQL on macOS...'

    # Install PostgreSQL using Homebrew
    if ! command -v brew &> /dev/null; then
        echo "Homebrew not found. Please install Homebrew first: https://brew.sh"
        exit 1
    fi

    brew install postgresql libpq

    # Start PostgreSQL service
    brew services start postgresql

    # Wait a moment for the service to start
    sleep 2

    # Create user and database (no sudo -u postgres needed on macOS)
    createuser $username
    createdb $database
    psql -d postgres -c "ALTER USER \"$username\" WITH PASSWORD '$password';"
    psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE \"$database\" TO \"$username\";"
    psql -d $database -c "GRANT ALL ON SCHEMA public TO \"$username\";"
    psql -d $database -c "GRANT CREATE ON SCHEMA public TO \"$username\";"
else
    # Linux setup (original)
    sudo apt-get install postgresql postgresql-contrib libpq-dev
    sudo service postgresql start
    sudo -u postgres createuser $username
    sudo -u postgres createdb $database
    sudo -u postgres psql -d postgres -c "ALTER USER \"$username\" WITH PASSWORD '$password';"
    sudo -u postgres psql -d postgres -c "GRANT ALL PRIVILEGES ON DATABASE \"$database\" TO \"$username\";"
fi
cargo install diesel_cli --no-default-features --features postgres
echo -e "DATABASE_URL=postgres://$username:$password@localhost/$database\nDIESEL_CONFIG_FILE=./diesel.toml\nINVITE_REQUIRED=true\nPORT=3000\nENVIRONMENT=development\nBCRYPT_COST=8" > backend/.env
echo -e "PUBLIC_VITE_API_URL=http://localhost:3000/api" > frontend/.env
cd backend
diesel setup
diesel migration run
cd ..

# TIME END
end=$(date +%s%N)

# CONGRATS 🎉
echo -e "\033[0;35m🪛 SETUP\033[0m completed in \033[0;32m$(($(($end-$start))/1000000)) ms\033[0m"

#!/bin/bash
set -e

echo "🔐 EnvSafe Container Starting..."

# Check if token is provided
if [ -z "$ENVSAFE_TOKEN" ]; then
    echo "❌ Error: ENVSAFE_TOKEN environment variable is required"
    exit 1
fi

# Login with token
echo "$ENVSAFE_TOKEN" | envsafe login -t

# Check if project is configured
if [ -n "$ENVSAFE_PROJECT_ID" ] && [ -n "$ENVSAFE_WORKSPACE_ID" ]; then
    # Create .envsafe config file
    cat > .envsafe <<EOF
{
  "workspace_id": "$ENVSAFE_WORKSPACE_ID",
  "project_id": "$ENVSAFE_PROJECT_ID",
  "project_name": "$ENVSAFE_PROJECT_NAME"
}
EOF
    echo "✓ Project configured"
fi

# Pull initial variables
echo "📥 Pulling environment variables..."
if [ -n "$ENVSAFE_ENV" ]; then
    envsafe pull --env "$ENVSAFE_ENV" --output .env
else
    envsafe pull --prod --output .env
fi

# Start watch mode if enabled
if [ "$ENVSAFE_WATCH" = "true" ]; then
    echo "👁️  Starting watch mode..."
    if [ -n "$ENVSAFE_ENV" ]; then
        envsafe watch --env "$ENVSAFE_ENV" &
    else
        envsafe watch --prod &
    fi
    
    # Wait for initial sync
    sleep 2
fi

echo "✓ EnvSafe initialized"
echo ""
echo "🚀 Starting application..."

# Execute the main command with injected variables
if [ -n "$ENVSAFE_ENV" ]; then
    exec envsafe run --env "$ENVSAFE_ENV" -- "$@"
else
    exec envsafe run --prod -- "$@"
fi

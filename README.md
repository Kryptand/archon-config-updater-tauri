# 🎮 Archon Talent Updater

A desktop application built with Tauri + Vue.js that automatically fetches and updates World of Warcraft talent builds from [Archon.gg](https://www.archon.gg) directly into your TalentLoadoutsEx addon.

## Features

- ✨ **Automatic Talent Fetching**: Fetches optimal talent builds from Archon.gg for any class/spec combination
- 🎯 **Raid & Mythic+ Support**: Handles both raid encounters (all difficulties) and Mythic+ dungeons
- 🔄 **Smart Fallback**: Automatically tries this-week/last-week data for M+ builds
- 💾 **Preserves User Builds**: Only manages auto-generated talents (marked with `_ARCT` suffix)
- 🌙 **Dark Mode**: Automatic dark mode support based on system preferences
- ⚡ **Fast & Efficient**: Rate-limited HTTP client with concurrent requests

## Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://www.rust-lang.org/) (latest stable)
- World of Warcraft with [TalentLoadoutsEx](https://www.curseforge.com/wow/addons/talent-loadouts-ex) addon installed

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd archon-config-updater-tauri
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Create your `settings.json` configuration file (see Configuration section below)

## Development

Run the development server:

```bash
npm run tauri dev
```

## Building

Build the application for production:

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`

## Configuration

Create a `settings.json` file based on `settings.example.json`:

```json
{
  "characters": [
    {
      "name": "MyWarrior",
      "class": "Warrior",
      "specializations": ["arms", "fury"]
    }
  ],
  "raidDifficulties": ["heroic", "normal"],
  "raidBosses": [
    "broodtwister",
    "sikran",
    "queen-ansurek"
  ],
  "dungeons": [
    "ara-kara",
    "city-of-threads",
    "mists-of-tirna-scithe"
  ],
  "clearPreviousBuilds": false,
  "outputPath": "/path/to/WoW/_retail_/WTF/Account/YOUR_ACCOUNT_ID/SavedVariables/TalentLoadoutsEx.lua"
}
```

### Configuration Options

| Option | Type | Description |
|--------|------|-------------|
| `characters` | Array | List of characters with class and specializations to fetch |
| `raidDifficulties` | Array | Raid difficulties to fetch (`"normal"`, `"heroic"`, `"mythic"`) |
| `raidBosses` | Array | Boss names (lowercase, hyphenated) |
| `dungeons` | Array | Dungeon names for M+ builds (lowercase, hyphenated) |
| `clearPreviousBuilds` | Boolean | Remove all auto-generated builds before updating |
| `outputPath` | String | Full path to your TalentLoadoutsEx.lua file |

### Finding Your outputPath

**macOS:**
```
/Applications/World of Warcraft/_retail_/WTF/Account/YOUR_ACCOUNT_ID/SavedVariables/TalentLoadoutsEx.lua
```

**Windows:**
```
C:\Program Files (x86)\World of Warcraft\_retail_\WTF\Account\YOUR_ACCOUNT_ID\SavedVariables\TalentLoadoutsEx.lua
```

**Linux:**
```
~/.wine/drive_c/Program Files (x86)/World of Warcraft/_retail_/WTF/Account/YOUR_ACCOUNT_ID/SavedVariables/TalentLoadoutsEx.lua
```

Replace `YOUR_ACCOUNT_ID` with your actual Battle.net account ID (it's a folder name, often numeric).

## Usage

1. Launch the application
2. Click "📁 Select settings.json" and choose your configuration file
3. Review the loaded configuration
4. Click "🚀 Update Talents" to fetch the latest builds
5. Wait for the process to complete
6. Launch WoW and check your TalentLoadoutsEx addon - new builds will have `_ARCT` suffix

## How It Works

1. **Fetch**: The app connects to Archon.gg and scrapes talent build data
2. **Parse**: Extracts Wowhead talent calculator links from the HTML
3. **Read**: Opens your TalentLoadoutsEx.lua file and parses existing talents
4. **Update**: Removes old auto-generated builds (with `_ARCT` suffix) and adds new ones
5. **Write**: Saves the updated talents back to the Lua file
6. **Preserve**: User-created talents (without `_ARCT`) are never modified

## Supported Classes

All 13 WoW classes are supported:
- Warrior, Paladin, Hunter, Rogue, Priest
- Death Knight, Shaman, Mage, Warlock, Monk
- Druid, Demon Hunter, Evoker

## Technical Stack

- **Frontend**: Vue.js 3 + TypeScript
- **Backend**: Rust (Tauri 2.0)
- **HTTP Client**: reqwest with rate limiting
- **HTML Parsing**: scraper (powered by selectors)
- **Lua Parsing**: full_moon
- **Async Runtime**: tokio

## Project Structure

```
archon-config-updater-tauri/
├── src/                    # Vue.js frontend
│   └── App.vue            # Main UI component
├── src-tauri/             # Rust backend
│   └── src/
│       ├── archon.rs      # URL building and identifiers
│       ├── config.rs      # Configuration parsing
│       ├── fetcher.rs     # HTTP client and HTML parser
│       ├── lua_talent.rs  # Lua file reading/writing
│       ├── orchestrator.rs # Main coordination logic
│       └── wow.rs         # WoW class/spec mappings
├── settings.example.json  # Example configuration
└── ARCHON_TALENT_FETCHING.md  # Detailed API documentation
```

## Troubleshooting

### "Failed to load config"
- Ensure your `settings.json` is valid JSON
- Check that all class names use PascalCase (e.g., `DeathKnight`, not `death-knight`)
- Verify specialization names are lowercase

### "Failed to write talents"
- Make sure WoW is not running (the file might be locked)
- Check the outputPath points to the correct location
- Verify you have write permissions to the SavedVariables folder

### "No talent build available"
- Some boss/spec combinations may not have data on Archon.gg yet
- This is normal for new content or unpopular specs
- The app will skip these and continue with others

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is for personal use. Archon.gg data is property of Archon.gg.

## Credits

- Built with [Tauri](https://tauri.app)
- Powered by [Vue.js](https://vuejs.org)
- Data from [Archon.gg](https://www.archon.gg)
- Compatible with [TalentLoadoutsEx](https://www.curseforge.com/wow/addons/talent-loadouts-ex) addon

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)

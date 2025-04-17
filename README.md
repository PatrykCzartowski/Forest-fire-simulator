# Forest-fire-simulator
A console-based forest fire simulation written in Rust that demonstrates how fires spread through different types of forests.


# 🌲 Features
* Dynamic forest generation with customizable size and density
* Different tree types with unique burning properties:
  * Pine: Highly flammable, burns quickly
  * Oak: Less flammable, burns slowly
  * Birch: Medium flammability and burn rate
  * Redwood: Fire resistant, burns very slowly
* Realistic natural water bodies that act as fire barriers
* Lightning strikes that initiate fires
* Visual representation of fire progression with colored text
* Statistics tracking for different tree types and burn percentages

# 📊 Simulation Mechanics
The simulation models several aspects of forest fires:
1. **Fire Spread**: Fires spread to adjacent trees based on each tree's flammability rating
2. **Burn Duration**: Different tree types burn for different amounts of time
3. **Water Barriers**: Natural water bodies in the forest prevent fire from spreading
4. **Fire Lifecycle**: Trees transition through several states:
   * Alive → Struck (by lightning) → Kindling → Burning → Burned

  
# 📋 Getting Started
Prerequisites
* Rust and Cargo (1.56.0 or higher)
Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/forest-fire-simulator.git
cd forest-fire-simulator
```
2. Build the project:
```bash
cargo build --release
```
3. Run the simulation:
```bash
cargo run --release
```


# 🎮 Usage
1. At startup, you'll be prompted to configure:
   * Forest size (10-100)
   * Forest density (0.1-1.0)
   * Water density (0.0-0.5)
2. Once the forest is generated:
   * Press Enter to simulate a lightning strike
   * Press 'q' to quit the simulation
3. Watch as the fire spreads based on tree types and forest composition


# 🎨 Display Legend
* Tree Types:
  * P = Pine (green)
  * O = Oak (green)
  * B = Birch (green)
  * R = Redwood (green)
* Fire States:
  * X = Struck by lightning (white)
  * K = Kindling (yellow)
  * F = Burning (red)
  * \# = Burned (grey)
* Terrain Types:
  * ~ = Water (blue)
  * . = Empty ground (gray)


# 🔧 Project Structure
* **main.rs**: Entry point and utility functions
* **config.rs**: Configuration settings and user input
* **tree.rs**: Tree types and their properties
* **tile.rs**: Forest tile representation
* **forest.rs**: Forest generation and display
* **simulation.rs**: Fire spread simulation logic

# 📄 License
This project is licensed under the MIT License - see the LICENSE file for details.

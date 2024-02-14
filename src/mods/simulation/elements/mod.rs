/*
    At a base level the most key elements for a cell (in this simulation) are
    - Carbon (Organic ,living things/hydrocarbons, and Inorganic, CO & CO2)
    - Oxygen ()
    - Hydrogen

    Compounds are made up of collections of materials:
    - C0, C02
    - H20
    - Carbohydrate

    Energy is created or stored through the process or transforming one compound into another:
    - Water + CO2 + Light(Energy) => Carbohydrate + O2 + H20
    - Carbohydrate => C02 + H20 + Energy / Waste,Water,Energy

    Access to elements comes mainly from the permeability of the membrane that allows for elements in the 'soup' to get inside the cell, but also may include:
    - Energy from the Light Source

    In order to achieve homeostasis (self-regulating sustenance) will need to both be able to:
    - Produce Carbohydrates (Synthesis)
    - Produce usable Energy (Respiration)

    Elements, Compounds and Energy have different levels of permutation both inter-cell and extra-cell:
    - Elements can flow freely between organelle neighbours but can be blocked by the membrane wall
    - Compounds can be moved between organelle neigbours or through a membrane wall but must be stored in special organelle
    - Energy cannot travel between organelle - it is either absorbed from the environment and synthesized or produced via respiration

    To simplify the simulation the following may be worth considering that the building blocks of the simulation are:
    - Elements
    - Compounds (Waste + Useful)

    Elements:
    - Fundamental : The basic building blocks of all organelle similar to carbon
    - Bond : A powerful bonding element who's addition to or removal from a compound requires or produces energy
    - Catalyst: A reactive element that wishes to bond with other elements, essential to kick off the process of synthesis or respiration

    Compounds:
    - SlowEnergy (F+B)
    - FastEnergy (R+F) Can be used by most organelle to power themselves but will overflow leading to waste
    - FundementalWaste(R+F) The result of FastEnergy overflowing + a byproduct of respiration
    - FundemntalMatter(F+F) A stable usable version of F
    -


    Iteration 2:

    The world is made up of the following:
        - Elements
        - Compounds
        - Natural Energy

    Elements:
    - Base - The basic building block of life, the most massive element.
    - Neutral - A neutral element that likes to bond with Base
    - Reactive - A reactive element that likes to modify existing compounds

    Compounds:
    - Water (Neutral + Reactive)
    - Store (Base + Neutral + Reactive): Requires energy to create, produces energy when reacted with Reactive
    - Active ([Energy] + Reactive)

    Reactions:
    - Photosynthesis: Base + Neutral + [Energy] => Store
    - Respiration: Store + Reactive => Active(Energy+Reactive) + DepletedBase + DepletedNeutral


    Iteration 3:

    The cycle of life is determined by the following two primary reactions:
    - Photosynthesis: The capture of energy from the sun in a stable compound
    - Respiration: The conversion of the energy is a compound from stable to Volatile

    Key compounds:
    - VolatileEnergy
    - StableEnergy

    Elements:
    - Bonder: A stable element that likes to bond with things
    - Mass: An element that does not like to bond with others
    - Agitator: An unstable element that like to force itself into Compounds

    Elemental Compounds:
    - Twin(Bonder): Exists naturally
    - Twin(Agitator): Exists naturally
    - Twin(Mass): Does not exist naturally

    Dual Compounds:
    - Waste(Mass+Agitator)
    - Fluid(Bonder+Agitator)

    Photosynthesis:
    - Takes a stable Waste, stable Fluid and energy are combined to break the Waste and Fluid apart resulting in a strong StableEnergy component (Bond+Mass) and Volatile a TwinAgitator
    - AM + BA + Energy => BM + AA

    Respiration:
    - Takes a StableEnergy(BM) and combines it with AA to produce VolitileEnergy (AB+Energy) and Waste (MA)
    - BM + AA + Energy => V(AB+E) + MA

    Energy Consumption
    - Individual Organelle can consume V(ABe) to power their actions and produce Waste(AB) as a byproduct


    Iteration #3:

    Elements:
    [A]ction (Oxygen)
    [B]ond (Hydrogen)
    [M]ass (Carbon)
    [e]nergy

    Compounds:
    - AA aka PairedAction
    - AB aka Fluid (makes up a lot of the world)
    - AM aka Waste (doesn't occur naturally)
    - BB aka PairedBond
    - BMe aka StableStore
    - ABe aka VolatileStore

    Impossible Compounds:
    - BM (won't bond without stored energy)
    - MM (won't bond with self)

*/

pub mod basic;
pub mod compounds;

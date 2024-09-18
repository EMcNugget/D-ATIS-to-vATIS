import { TAirportData, TATISType } from "../lib/types";

const get_direction = (heading: number) => {
  const directions = [
    "NORTH",
    "NORTHEAST",
    "EAST",
    "SOUTHEAST",
    "SOUTH",
    "SOUTHWEST",
    "WEST",
    "NORTHWEST",
  ];
  return directions[Math.round(heading / 45) % 8];
};

const get_runways = (airport_conditions: string) => {
  const strs = airport_conditions.split(/[\s,.]+/);
  const runway_regex = /^[0-9]{1,2}(L|C|R)?$/;

  const rwy_set = Array.from(
    new Set(strs.filter((str) => runway_regex.test(str)))
  );

  return rwy_set.map((rwy) => {
    if (rwy.length < 3 && !isNaN(parseInt(rwy))) {
      return `0${rwy}`;
    }
  });
};

export const get_flow = async (
  airport_conditions: string,
  airport_data: TAirportData
) => {
  const runways = get_runways(airport_conditions);
  const directions: string[] = [];

  airport_data.runways.forEach((rwy) => {
    if (runways.includes(rwy.he_ident)) {
      directions.push(get_direction(parseInt(rwy.he_heading_degT)));
    } else if (runways.includes(rwy.le_ident)) {
      directions.push(get_direction(parseInt(rwy.le_heading_degT)));
    } else return;
  });

  directions.sort();
  return directions[0];
};

const app_types = ["VISUAL", "VIS", "VOR", "RNAV", "ILS", "GLS", "LPV"];

export const get_preset_name = async (
  airport_conditions: string,
  airport_data: TAirportData,
  type: TATISType
) => {
  const flow = await get_flow(airport_conditions, airport_data);
  let app_type = app_types.find((type) => airport_conditions.includes(type));
  let dep_type = airport_conditions.includes("RNAV") ? "ROTG" : "";
  if (app_type === "VISUAL") {
    app_type = "VIS";
  }

  if (type === "combined") {
    return `REAL WORLD: ${flow} ${app_type} ${dep_type}`;
  } else if (type === "arr") {
    return `REAL WORLD: ${flow} ${app_type}`;
  } else if (type === "dep") {
    return `REAL WORLD: ${dep_type}`;
  }
};

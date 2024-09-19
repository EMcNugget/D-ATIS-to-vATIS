import { invoke } from "@tauri-apps/api/core";
import { TATISType, TRunways } from "../lib/types";

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
  facility: string,
  airport_conditions: string
) => {
  const runways = get_runways(airport_conditions);
  const directions: string[] = [];
  const runway_data = (await invoke<Promise<TRunways>>("get_runways"))[
    facility
  ];

  runways.forEach((rwy) => {
    const runway = runway_data.find((runway) => runway.runway_id === rwy);
    if (runway) {
      directions.push(get_direction(runway.heading));
    }
  });

  directions.sort();
  return directions[0];
};

const app_types = ["VISUAL", "VIS", "VOR", "RNAV", "ILS", "GLS", "LPV"];

export const get_preset_name = async (
  airport_conditions: string,
  facility: string,
  type: TATISType
) => {
  const flow = await get_flow(facility, airport_conditions);
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
  } else {
    return "REAL WORLD";
  }
};

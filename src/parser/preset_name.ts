import { TAirportData } from "../lib/types";

const get_direction = (heading: number) => {
  const directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
  return directions[Math.round(heading / 45) % 8];
};

const get_runways = (airport_conditions: string) => {
  const strs = airport_conditions.split(/[\s,.]+/);
  const runway_regex = /^[0-9]{1,2}(L|C|R)?$/;

  const rwy_set = new Set(strs.filter((str) => runway_regex.test(str)));
  return Array.from(rwy_set);
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

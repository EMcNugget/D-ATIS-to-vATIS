import type { ATIS, vATIS } from "./types.js";
import { use_atis_store } from "./stores.js";
import { v4 } from "uuid";
import { computed } from "vue";

const store = use_atis_store();

const facility = computed(() => store.get_facility());

const find_number_of_occurances = (
  str: string,
  char: string,
  n: number
): number => {
  return (
    str.split("").reduce((acc: number[], cur, index) => {
      if (cur === char) {
        acc.push(index);
      }
      return acc;
    }, [])[n - 1] || -1
  );
};

const create_template = (
  facility: string,
  combined: boolean,
  type?: "ARR" | "DEP"
): string => {
  return `${facility}${
    combined ? ` ${type}` : ""
  } ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
};

const parse_atis = (atis: ATIS, split: boolean): vATIS => {
  const notams = atis.datis.split("NOTAMS... ")[1].split(" ...ADVS")[0];
  const airport_conditions = atis.datis
    .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
    .split(
      atis.datis.includes("NOTAMS") ? "NOTAMS..." : "NOTICE TO AIR MISSIONS."
    )[0]
    .trim();

  return {
    id: v4(),
    name: "REAL WOLRD",
    airport_conditions,
    notams,
    template: create_template(
      facility.value,
      split,
      split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
    ),
    external_generator: {
      enabled: false, // not sure what this does, leaving it as false for now
    },
  };
};

export const fetch_atis = async () => {
  const response = await fetch(`https://datis.clowd.io/api/k${facility}`);
  const data = (await response.json()) as ATIS[];

  const atisArray: vATIS[] = [];
  data.forEach((atis) => {
    atisArray.push(parse_atis(atis, false));
  });

  return atisArray;
};

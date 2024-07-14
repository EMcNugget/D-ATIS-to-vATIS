import type { ATIS, vATIS } from "./types.js";
import { useATISSTore } from "./stores.js";
import { v4 } from "uuid";
import { computed } from "vue";

const store = useATISSTore();

const facility = computed(() => store.get_facility());

const findNthOccurrenceIndex = (
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

const createTemplate = (
  facility: string,
  combined: boolean,
  type?: "ARR" | "DEP"
): string => {
  return `${facility}${
    combined ? ` ${type}` : ""
  } ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
};

const parseATIS = (atis: ATIS, split: boolean): vATIS => {
  const notams = atis.datis.split("NOTAMS... ")[1].split(" ...ADVS")[0];
  const airport_conditions = atis.datis
    .slice(findNthOccurrenceIndex(atis.datis, ".", 2) + 1)
    .split(
      atis.datis.includes("NOTAMS") ? "NOTAMS..." : "NOTICE TO AIR MISSIONS."
    )[0]
    .trim();

  return {
    id: v4(),
    name: "REAL WOLRD",
    airport_conditions,
    notams,
    template: createTemplate(
      facility.value,
      split,
      split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
    ),
    external_generator: {
      enabled: false, // not sure what this does, leaving it as false for now
    },
  };
};

export const fetchATIS = async () => {
  const response = await fetch(`https://datis.clowd.io/api/k${facility}`);
  const data = (await response.json()) as ATIS[];

  const atisArray = [];

  if (data.length === 1) {
    atisArray.push(parseATIS(data[0], false));
  } else {
    atisArray.push(parseATIS(data[0], true));
    atisArray.push(parseATIS(data[1], true));
  }

  return atisArray;
};

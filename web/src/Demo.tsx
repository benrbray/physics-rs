import { initGame } from "@lib/main";
import { onMount } from "solid-js";

export const Demo = () => {
  let demoElt!: HTMLCanvasElement;

  onMount(() => {
    initGame(demoElt);
  });

  return <canvas tabIndex={1} ref={demoElt} />
}
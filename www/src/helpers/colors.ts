const invert_color = (hex: string) => {
  if (hex.indexOf("#") === 0) {
    hex = hex.slice(1);
  }
  if (hex.length === 3) {
    hex = hex[0] + hex[0] + hex[1] + hex[1] + hex[2] + hex[2];
  }
  let r, g, b;
  if (hex.slice(0, 3) === "rgb") {
    const colors = hex.match(/.*\((\d+), (\d+), (\d+)(, \d+)?\)/)!;
    r = parseInt(colors[1]);
    g = parseInt(colors[2]);
    b = parseInt(colors[3]);
  } else {
    r = parseInt(hex.slice(0, 2), 16);
    g = parseInt(hex.slice(2, 4), 16);
    b = parseInt(hex.slice(4, 6), 16);
  }
  return r * 0.299 + g * 0.587 + b * 0.114 > 186 ? "#000000" : "#FFFFFF";
};

export default invert_color;

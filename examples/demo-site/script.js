const countEl = document.getElementById("count");
const statusEl = document.getElementById("status");

let count = Number(localStorage.getItem("count") || 0);
render();

document.getElementById("plus").addEventListener("click", () => {
  count++;
  localStorage.setItem("count", count);
  render();
});

document.getElementById("reset").addEventListener("click", () => {
  count = 0;
  localStorage.setItem("count", count);
  render();
});

document.getElementById("theme").addEventListener("click", () => {
  const dark = document.body.classList.toggle("dark");
  localStorage.setItem("dark", dark);
});

if (localStorage.getItem("dark") === "true") {
  document.body.classList.add("dark");
}

function render() {
  countEl.textContent = count;
  statusEl.textContent = "JavaScript is running. Value is saved on this device.";
}

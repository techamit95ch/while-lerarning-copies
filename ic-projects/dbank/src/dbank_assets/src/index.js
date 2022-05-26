import { dbank } from "../../declarations/dbank";

window.addEventListener("load", async function () {
  const currentAmmount = await dbank.getBalance();

  document.getElementById("balance").innerText =
    Math.round(currentAmmount * 100) / 100;
});

document.querySelector("form").addEventListener("submit", async function (e) {
  e.preventDefault();
  const button = document.querySelector("button");
  button.setAttribute("disabled", true);

  const withdrawl = document.getElementById("withdrawl").value;
  const deposit = document.getElementById("deposit").value;
  if (
    withdrawl &&
    withdrawl.length !== 0 &&
    withdrawl.length !== NaN &&
    withdrawl > 0
  ) {
    await dbank.withDrawl(Number(withdrawl));
  }
  if (
    deposit &&
    deposit.length !== 0 &&
    deposit.length !== NaN &&
    deposit > 0
  ) {
    await dbank.toUp(Number(deposit));
  }
  await dbank.compound();
  document.getElementById("balance").innerText =
    Math.round((await dbank.getBalance()) * 100) / 100;
  button.removeAttribute("disabled");
});

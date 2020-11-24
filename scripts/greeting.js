export default function() {
  let greetings = document.querySelectorAll(".greeting");
  let today = new Date();
  let current_hour = today.getHours();
  let day_region;
  if (current_hour < 12) {
    day_region = "morning"
  } else if (current_hour < 17) {
    day_region = "afternoon"
  } else {
    day_region = "evening"
  }
  greetings.forEach((greet) => {
    greet.innerHTML = `Good ${day_region}`;
  });
}

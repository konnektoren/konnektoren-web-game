console.log(window.konnektoren);

const data =
  window.konnektoren && window.konnektoren.challenge
    ? window.konnektoren.challenge.data
    : new Map([
        [
          "endings",
          [
            new Map([
              ["ending", "-chen"],
              ["article", "das"],
            ]),
            new Map([
              ["ending", "-lein"],
              ["article", "das"],
            ]),
            new Map([
              ["ending", "-ung"],
              ["article", "die"],
            ]),
            new Map([
              ["ending", "-heit"],
              ["article", "die"],
            ]),
            new Map([
              ["ending", "-er"],
              ["article", "der"],
            ]),
            // ... add more endings as needed
          ],
        ],
      ]);

let currentEndingIndex = 0;
let correctAnswers = 0;
let totalQuestions = 0;
let userAnswers = [];

function translateStaticText() {
  const title = document.querySelector(
    ".custom_group_of_nouns-container__title",
  );
  const finishButton = document.querySelector(
    ".custom_group_of_nouns-container__finish-button",
  );

  if (title) {
    title.textContent = window.konnektoren.tr(
      "Identify the German Noun Gender",
    );
  }
  if (finishButton) {
    finishButton.textContent = window.konnektoren.tr("Finish Exercise");
  }
}

function displayEnding() {
  const endings = data.get("endings");
  const currentEnding = endings[currentEndingIndex];
  const endingDisplayElement = document.getElementById("ending-display");
  if (endingDisplayElement) {
    endingDisplayElement.textContent = `${window.konnektoren.tr("Noun ending:")} ${currentEnding.get("ending")}`;

    document.getElementById("feedback").textContent = "";
    document.getElementById("correct-answer").textContent = "";
  }
}

function checkAnswer(selectedArticle) {
  const endings = data.get("endings");
  const currentEnding = endings[currentEndingIndex];
  const correctArticle = currentEnding.get("article");
  const feedbackElement = document.getElementById("feedback");
  const correctAnswerElement = document.getElementById("correct-answer");

  totalQuestions++;
  let isCorrect = selectedArticle === correctArticle;

  if (isCorrect) {
    correctAnswers++;
    feedbackElement.textContent = window.konnektoren.tr("Correct!");
    document.body.classList.add("custom_group_of_nouns-container--green-bg");
  } else {
    feedbackElement.textContent = window.konnektoren.tr("Incorrect!");
    document.body.classList.add("custom_group_of_nouns-container--red-bg");
  }

  correctAnswerElement.textContent = `${window.konnektoren.tr("The correct answer is:")} ${correctArticle}`;

  userAnswers.push({
    ending: currentEnding.get("ending"),
    selectedArticle: selectedArticle,
    correctArticle: correctArticle,
    isCorrect: isCorrect,
  });

  setTimeout(() => {
    document.body.classList.remove(
      "custom_group_of_nouns-container--green-bg",
      "custom_group_of_nouns-container--red-bg",
    );
    currentEndingIndex++;
    if (currentEndingIndex >= data.get("endings").length) {
      finishChallenge();
    } else {
      displayEnding();
    }
  }, 2000);
}

function finishChallenge() {
  const event = {
    type: "Finish",
    result: {
      id: window.konnektoren.challenge.id,
      performance: calculatePerformance(),
      data: {
        answers: userAnswers,
      },
    },
  };
  window.konnektoren.sendEvent(event);
}

function calculatePerformance() {
  return correctAnswers / data.get("endings").length;
}

function displayResults() {
  const resultsContainer = document.querySelector(
    ".custom_group_of_nouns-results-container",
  );
  if (!resultsContainer) {
    console.warn("Results container not found.");
    return;
  }

  resultsContainer.innerHTML = "";

  const result = window.konnektoren && window.konnektoren.result;
  if (!result) {
    console.warn("No result data available.");
    return;
  }

  const performance = result.performance;
  console.log(result);
  const answersMap = result.data.get("answers");

  console.log(answersMap);

  if (!answersMap || !Array.isArray(answersMap)) {
    console.warn("Invalid answers data.");
    return;
  }

  try {
    const performanceElement = document.createElement("h2");
    performanceElement.textContent = `${window.konnektoren.tr("Your Performance:")}: ${(performance * 100).toFixed(2)}%`;
    resultsContainer.appendChild(performanceElement);

    const overviewElement = document.createElement("div");
    const correctAnswers = answersMap.filter((answer) =>
      answer.get("isCorrect"),
    ).length;
    const totalQuestions = data.get("endings").length;

    overviewElement.innerHTML = `
      <h3>${window.konnektoren.tr("Overview")}</h3>
      <p>${window.konnektoren.tr("Correct Answers")}: ${correctAnswers}</p>
      <p>${window.konnektoren.tr("Incorrect Answers")}: ${totalQuestions - correctAnswers}</p>
    `;
    resultsContainer.appendChild(overviewElement);

    const detailedResultsElement = document.createElement("div");
    detailedResultsElement.innerHTML = `<h3>${window.konnektoren.tr("Detailed Results")}</h3>`;
    const resultsList = document.createElement("ul");

    answersMap.forEach((answer) => {
      if (answer && typeof answer.get === "function") {
        const listItem = document.createElement("li");
        listItem.innerHTML = `
          <p>${window.konnektoren.tr("Ending:")}: ${answer.get("ending") || "N/A"}</p>
          <p>${window.konnektoren.tr("Your Answer:")}: ${answer.get("selectedArticle") || "N/A"}</p>
          <p>${window.konnektoren.tr("Correct Answer:")}: ${answer.get("correctArticle") || "N/A"}</p>
          <p>${answer.get("isCorrect") ? "✅" : "❌"}</p>
        `;
        resultsList.appendChild(listItem);
      } else {
        console.warn("Invalid answer data:", answer);
      }
    });

    detailedResultsElement.appendChild(resultsList);
    resultsContainer.appendChild(detailedResultsElement);
  } catch (error) {
    console.error("Error displaying results:", error);
  }
}

function init() {
  if (!window.konnektoren.result) {
    translateStaticText();
    displayEnding();

    const choiceItems = document.querySelectorAll(
      ".custom_group_of_nouns-container__choice-item",
    );
    choiceItems.forEach((item) => {
      item.addEventListener("click", function () {
        checkAnswer(this.dataset.article);
      });
    });

    const finishButton = document.querySelector(
      ".custom_group_of_nouns-container__finish-button",
    );
    if (finishButton) {
      finishButton.addEventListener("click", finishChallenge);
    }
  } else {
    displayResults();
  }
}

// Initialize the challenge
init();

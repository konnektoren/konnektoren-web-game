console.log(window.konnektoren);

const data = window.konnektoren
  ? window.konnektoren.challenge.data
  : new Map([
      [
        "questions",
        [
          new Map([
            ["question", "Der Hund läuft im Park. Welcher Fall?"],
            ["options", ["Nominativ", "Akkusativ", "Dativ", "Genitiv"]],
            ["correct", "Nominativ"],
          ]),
          new Map([
            ["question", "Ich sehe den Apfel. Welcher Fall?"],
            ["options", ["Nominativ", "Akkusativ", "Dativ", "Genitiv"]],
            ["correct", "Akkusativ"],
          ]),
          new Map([
            ["question", "Das ist das Haus des Mannes. Welcher Fall?"],
            ["options", ["Nominativ", "Akkusativ", "Dativ", "Genitiv"]],
            ["correct", "Genitiv"],
          ]),
          new Map([
            ["question", "Ich gebe dem Kind ein Geschenk. Welcher Fall?"],
            ["options", ["Nominativ", "Akkusativ", "Dativ", "Genitiv"]],
            ["correct", "Dativ"],
          ]),
        ],
      ],
    ]);

console.log(data);

let currentQuestionIndex = 0;
let correctAnswers = 0;
let userAnswers = [];

function translateStaticText() {
  const title = document.querySelector(".custom-casus-challenge__title");
  const finishButton = document.querySelector(
    ".custom-casus-challenge__button--finish",
  );

  if (finishButton === null) {
    return;
  }

  if (title && window.konnektoren) {
    title.textContent = window.konnektoren.tr("German Casus Exercise");
  }

  if (finishButton && window.konnektoren) {
    finishButton.textContent = window.konnektoren.tr("Finish Exercise");
  }
}

function finishChallenge() {
  if (window.konnektoren && window.konnektoren.sendEvent) {
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
  } else {
    console.log(
      "Quiz Finished! Your performance: " + calculatePerformance() * 100 + "%",
    );
    console.log("User Answers:", userAnswers);
    alert(
      "Quiz Finished! Your performance: " + calculatePerformance() * 100 + "%",
    );
  }
  displayResults();
}

function loadQuestion() {
  const currentQuestion = data.get("questions")[currentQuestionIndex];
  const optionsList = document.querySelector(
    ".custom-casus-challenge__options-list",
  );

  if (optionsList === null) {
    return;
  }

  const feedback = document.querySelector(".custom-casus-challenge__feedback");

  feedback.textContent = "";
  feedback.style.display = "none";

  // Clear previous question content
  optionsList.innerHTML = "";

  // Create the question text element dynamically
  const questionText = document.createElement("p");
  questionText.className = "custom-casus-challenge__question-text";
  questionText.textContent = currentQuestion.get("question");
  optionsList.appendChild(questionText); // Append the question text first

  // Create and display options
  currentQuestion.get("options").forEach((option, index) => {
    const li = document.createElement("li");
    li.className = "custom-casus-challenge__option";
    li.innerHTML = `
      <label class="custom-casus-challenge__option-label">
        <input type="radio" name="q${currentQuestionIndex}" value="${option.toLowerCase()}" class="custom-casus-challenge__option-input">
        ${option}
      </label>
    `;
    optionsList.appendChild(li);
    // Show the list items
    li.style.display = "block";
  });

  // Add a submit button after options
  const submitButton = document.createElement("button");
  submitButton.type = "submit";
  submitButton.className = "custom-casus-challenge__submit-button";
  submitButton.textContent = "Antworten Überprüfen";
  optionsList.appendChild(submitButton);

  // Attach an event listener to the submit button
  submitButton.addEventListener("click", function (event) {
    event.preventDefault();
    checkAnswer();
  });
}

function nextQuestion() {
  currentQuestionIndex++;
  loadQuestion();
}

function checkAnswer() {
  const currentQuestion = data.get("questions")[currentQuestionIndex];
  const selectedOption = document.querySelector(
    `input[name="q${currentQuestionIndex}"]:checked`,
  );
  const feedback = document.querySelector(".custom-casus-challenge__feedback");
  let isCorrect = false;

  if (
    selectedOption &&
    selectedOption.value === currentQuestion.get("correct").toLowerCase()
  ) {
    correctAnswers++;
    isCorrect = true;
    feedback.textContent = window.konnektoren
      ? window.konnektoren.tr("Correct!")
      : "Correct!";
    feedback.classList.add("custom-casus-challenge__feedback--correct");
    feedback.classList.remove("custom-casus-challenge__feedback--incorrect");
  } else {
    feedback.textContent = window.konnektoren
      ? window.konnektoren.tr("Incorrect!")
      : "Incorrect!";
    feedback.classList.add("custom-casus-challenge__feedback--incorrect");
    feedback.classList.remove("custom-casus-challenge__feedback--correct");
  }

  userAnswers.push({
    questionId: currentQuestionIndex,
    selectedOption: selectedOption ? selectedOption.value : null,
    correctOption: currentQuestion.get("correct"),
    isCorrect: isCorrect,
  });

  feedback.style.display = "inline-block";

  if (currentQuestionIndex === data.get("questions").length - 1) {
    finishChallenge();
  } else {
    setTimeout(() => {
      feedback.style.display = "none";
      nextQuestion();
    }, 1000);
  }
}

function calculatePerformance() {
  return correctAnswers / data.get("questions").length;
}

function displayResults() {
  const resultsContainer = document.querySelector(
    ".custom-casus-challenge__results-container",
  );

  if (!resultsContainer) {
    console.warn("Results container not found.");
    return;
  }

  resultsContainer.innerHTML = "";

  const performanceWrapper = document.createElement("p");
  performanceWrapper.className = "custom-casus-challenge__performance-wrapper";
  performanceWrapper.innerHTML = `
    ${window.konnektoren ? window.konnektoren.tr("Your performance") : "Your performance"}:
    <span class="custom-casus-challenge__performance">${(
      calculatePerformance() * 100
    ).toFixed(2)}%</span>
  `;
  resultsContainer.appendChild(performanceWrapper);

  const userAnswers =
    window.konnektoren &&
    window.konnektoren.result &&
    window.konnektoren.result.data.get("answers");

  console.log("user answers", userAnswers);

  if (userAnswers) {
    userAnswers.forEach((answer) => {
      if (answer.get("isCorrect") === true) {
        correctAnswers++;
      }
    });
  }

  const overviewSection = document.createElement("div");
  overviewSection.className = "custom-casus-challenge__overview";
  overviewSection.innerHTML = `
    <h3 class="custom-casus-challenge__overview-title">${
      window.konnektoren ? window.konnektoren.tr("Overview") : "Overview"
    }</h3>
    <p class="custom-casus-challenge__overview-item custom-casus-challenge__overview-item--correct">${
      window.konnektoren
        ? window.konnektoren.tr("Correct Answers")
        : "Correct Answers"
    }: ${correctAnswers}</p>
    <p class="custom-casus-challenge__overview-item custom-casus-challenge__overview-item--incorrect">${
      window.konnektoren
        ? window.konnektoren.tr("Incorrect Answers")
        : "Incorrect Answers"
    }: ${data.get("questions").length - correctAnswers}</p>
  `;
  resultsContainer.appendChild(overviewSection);

  const resultsList = document.createElement("ul");
  resultsList.className = "custom-casus-challenge__results-list";

  // Access the userAnswers from the results data

  if (userAnswers) {
    userAnswers.forEach((answer, index) => {
      const question = data.get("questions")[index];
      const listItem = document.createElement("li");
      listItem.className = "custom-casus-challenge__result-item";
      listItem.innerHTML = `
        <p class="custom-casus-challenge__result-question"><strong>${
          window.konnektoren ? window.konnektoren.tr("Question") : "Question"
        }:</strong> ${question.get("question")}</p>
        <p class="custom-casus-challenge__result-answer"><strong>${
          window.konnektoren
            ? window.konnektoren.tr("Your answer")
            : "Your answer"
        }:</strong> ${answer.get("selectedOption") || "Not answered"} ${
          answer.get("isCorrect") ? "✅" : "❌"
        }</p>
        <p class="custom-casus-challenge__result-correct"><strong>${
          window.konnektoren
            ? window.konnektoren.tr("Correct answer")
            : "Correct answer"
        }:</strong> ${question.get("correct")}</p>
      `;
      resultsList.appendChild(listItem);
    });
  } else {
    console.warn("User answers not found in results data.");
  }

  resultsContainer.appendChild(resultsList);
}

const submitButton = document.querySelector(
  ".custom-casus-challenge__submit-button",
);

if (submitButton) {
  submitButton.addEventListener("click", function (event) {
    event.preventDefault();
    checkAnswer();
  });
}

const finishButton = document.querySelector(
  ".custom-casus-challenge__button--finish",
);

if (finishButton) {
  finishButton.addEventListener("click", function () {
    finishChallenge();
  });
}

// Automatically load the first question on page load
translateStaticText();
loadQuestion();

// Call the function to display results
displayResults();

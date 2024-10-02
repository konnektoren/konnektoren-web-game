console.log(window.konnektoren);

const data = window.konnektoren
  ? window.konnektoren.challenge.data
  : new Map([
      [
        "options",
        [
          new Map([
            ["id", 0],
            ["name", "der"],
          ]),
          new Map([
            ["id", 1],
            ["name", "die"],
          ]),
          new Map([
            ["id", 2],
            ["name", "das"],
          ]),
        ],
      ],
      [
        "questions",
        [
          new Map([
            ["question", "What is the article for 'Haus'?"],
            ["option", 2], // 'das' is correct
            ["help", "The article for 'Haus' is 'das'."],
          ]),
          new Map([
            ["question", "What is the article for 'Tisch'?"],
            ["option", 0], // 'der' is correct
            ["help", "The article for 'Tisch' is 'der'."],
          ]),
          new Map([
            ["question", "What is the article for 'Buch'?"],
            ["option", 2], // 'das' is correct
            ["help", "The article for 'Buch' is 'das'."],
          ]),
        ],
      ],
    ]);

console.log(data);

let currentQuestionIndex = 0;
let correctAnswers = 0;
let userAnswers = [];

// Translate static text on page load
function translateStaticText() {
  const title = document.querySelector(".custom-articles-challenge__title");
  const finishButton = document.querySelector(
    ".custom-articles-challenge__button--finish",
  );

  if (title) {
    title.textContent = window.konnektoren.tr("German Article Exercise");
  }

  if (finishButton) {
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
    // For testing purposes, display the results
    console.log(
      "Quiz Finished! Your performance: " + calculatePerformance() * 100 + "%",
    );
    console.log("User Answers:", userAnswers);
    alert(
      "Quiz Finished! Your performance: " + calculatePerformance() * 100 + "%",
    );
  }
}

// Load the current question
function loadQuestion() {
  const currentQuestion = data.get("questions")[currentQuestionIndex];
  const questionText = document.querySelector(
    ".custom-articles-challenge__question-text",
  );

  if (questionText === null) {
    return;
  }

  const helpText = document.querySelector(
    ".custom-articles-challenge__help-text",
  );
  const questionImage = document.querySelector(
    ".custom-articles-challenge__question-image",
  );
  const optionsList = document.querySelector(
    ".custom-articles-challenge__options-list",
  );
  const feedback = document.querySelector(
    ".custom-articles-challenge__feedback",
  );

  // Reset feedback on new question
  feedback.textContent = "";
  feedback.style.display = "none";

  // Update question and help text
  questionText.textContent = currentQuestion.get("question");
  helpText.textContent = currentQuestion.get("help");

  // Update question image if available
  if (currentQuestion.get("image")) {
    questionImage.className =
      "custom-articles-challenge__question-image fas " +
      currentQuestion.get("image");
  } else {
    questionImage.className = "custom-articles-challenge__question-image";
  }

  // Clear previous options
  optionsList.innerHTML = "";

  // Create and display options
  data.get("options").forEach((option) => {
    const li = document.createElement("li");
    li.className = "custom-articles-challenge__option";
    li.textContent = option.get("name");
    li.dataset.id = option.get("id");

    // Handle option click
    li.addEventListener("click", () => {
      checkAnswer(option.get("id"));
    });

    optionsList.appendChild(li);
  });
}

function nextQuestion() {
  currentQuestionIndex++;
  loadQuestion();
}

// Check if the selected answer is correct
function checkAnswer(selectedOption) {
  const currentQuestion = data.get("questions")[currentQuestionIndex];
  const feedback = document.querySelector(
    ".custom-articles-challenge__feedback",
  );
  let isCorrect = false;

  // Show feedback based on whether the answer is correct or not
  if (selectedOption === currentQuestion.get("option")) {
    correctAnswers++;
    isCorrect = true;
    feedback.textContent = window.konnektoren.tr("Correct!");
    feedback.classList.add("custom-articles-challenge__feedback--correct");
    feedback.classList.remove("custom-articles-challenge__feedback--incorrect");
  } else {
    feedback.textContent = window.konnektoren.tr("Incorrect!");
    feedback.classList.add("custom-articles-challenge__feedback--incorrect");
    feedback.classList.remove("custom-articles-challenge__feedback--correct");
  }

  // Record user's answer
  userAnswers.push({
    questionId: currentQuestion.id,
    selectedOption: selectedOption,
    correctOption: currentQuestion.option,
    isCorrect: isCorrect,
  });

  feedback.style.display = "inline-block"; // This line now directly shows the feedback

  if (currentQuestionIndex === data.get("questions").length - 1) {
    finishChallenge();
    return;
  } else {
    setTimeout(() => {
      feedback.style.display = "none"; // Hide feedback before moving to next question
      nextQuestion();
    }, 1000);
  }
}

function calculatePerformance() {
  const totalQuestions = data.get("questions").length;
  const performance = correctAnswers / totalQuestions;
  return performance;
}

// Automatically load the first question on page load
translateStaticText();
loadQuestion();

const finishButton = document.querySelector(
  ".custom-articles-challenge__button--finish",
);

if (finishButton) {
  finishButton.addEventListener("click", function () {
    finishChallenge();
  });
}

function displayResults() {
  // Check if window.konnektoren and result data are available
  const result = window.konnektoren && window.konnektoren.result;

  if (!result) {
    console.warn("No result data available in window.konnektoren.result.");
    return; // Exit the function if no result data
  }

  // Access the challenge data (questions and options)
  const challengeData =
    window.konnektoren &&
    window.konnektoren.challenge &&
    window.konnektoren.challenge.data;

  if (!challengeData) {
    console.warn(
      "No challenge data available in window.konnektoren.challenge.data.",
    );
    return; // Exit the function if no challenge data
  }

  // Get the performance and answers
  const performance = result.performance;
  const answersMap = result.data.get("answers");

  console.log("answers", answersMap);

  // Try to get the performance wrapper element
  const performanceWrapper = document.querySelector(
    ".custom-articles-challenge__performance-wrapper",
  );

  if (performanceWrapper) {
    // Update the text content with the translated version
    performanceWrapper.innerHTML = `
      ${window.konnektoren.tr("Your performance")}:
      <span class="custom-articles-challenge__performance"></span>%
    `;

    // Get the performance span element
    const performanceElement = performanceWrapper.querySelector(
      ".custom-articles-challenge__performance",
    );

    if (performanceElement) {
      // Display performance percentage
      performanceElement.textContent = (performance * 100).toFixed(2);
    } else {
      console.warn(
        'Element with class "custom-articles-challenge__performance" not found.',
      );
    }
  } else {
    console.warn(
      'Element with class "custom-articles-challenge__performance-wrapper" not found.',
    );
  }

  // Try to get the results container element
  const resultsContainer = document.querySelector(
    ".custom-articles-challenge__results-container",
  );

  if (resultsContainer) {
    let correctCount = 0;
    let incorrectCount = 0;

    // Create overview section
    const overviewSection = document.createElement("div");
    overviewSection.className = "custom-articles-challenge__overview";

    // Create detailed results list
    const resultsList = document.createElement("ul");
    resultsList.className = "custom-articles-challenge__results-list";

    // Extract answers, questions, and options
    const answers = Array.from(answersMap);
    const questions = Array.from(challengeData.get("questions"));
    const options = Array.from(challengeData.get("options"));

    // Iterate through answers using index
    answers.forEach((answer, index) => {
      // Get the corresponding question from the array using the index
      const question = questions[index]; // Use index to access the question

      // Check if the question is found
      if (question) {
        const selectedOption = options.find(
          (o) => o.get("id") === answer.get("selectedOption"),
        );
        const correctOption = options.find(
          (o) => o.get("id") === question.get("option"),
        );

        if (answer.get("isCorrect")) {
          correctCount++;
        } else {
          incorrectCount++;
        }

        const listItem = document.createElement("li");
        listItem.className = "custom-articles-challenge__result-item";

        listItem.innerHTML = `
          <p class="custom-articles-challenge__result-question"><strong>${window.konnektoren.tr("Question")}:</strong> ${question.get("question")}</p>
          <p class="custom-articles-challenge__result-answer"><strong>${window.konnektoren.tr("Your answer")}:</strong> ${selectedOption ? selectedOption.get("name") : "Option not found"} ${answer.get("isCorrect") ? "✅" : "❌"}</p>
          <p class="custom-articles-challenge__result-correct"><strong>${window.konnektoren.tr("Correct answer")}:</strong> ${correctOption ? correctOption.get("name") : "Option not found"}</p>
          <p class="custom-articles-challenge__result-help"><strong>${window.konnektoren.tr("Help")}:</strong> ${question.get("help")}</p>
        `;

        resultsList.appendChild(listItem);
      } else {
        console.warn("Question not found at index:", index);
      }
    });

    // Add overview to the overview section
    overviewSection.innerHTML = `
      <h3 class="custom-articles-challenge__overview-title">${window.konnektoren.tr("Overview")}</h3>
      <p class="custom-articles-challenge__overview-item custom-articles-challenge__overview-item--correct">${window.konnektoren.tr("Correct Answers")}: ${correctCount}</p>
      <p class="custom-articles-challenge__overview-item custom-articles-challenge__overview-item--incorrect">${window.konnektoren.tr("Incorrect Answers")}: ${incorrectCount}</p>
    `;

    // Append overview and results list to the results container
    resultsContainer.appendChild(overviewSection);
    resultsContainer.appendChild(resultsList);
  } else {
    console.warn(
      'Element with class "custom-articles-challenge__results-container" not found.',
    );
  }
}

// Call the function to display results
displayResults();

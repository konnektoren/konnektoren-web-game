function checkExercise1() {
    var answer = document.getElementById("exercise1").value.trim().toLowerCase();
    var feedback = document.getElementById("feedback1");

    // Update feedback each time, allowing multiple attempts
    feedback.textContent = '';  // Clear feedback on each submission

    if (answer === "musst") {
        feedback.textContent = "Correct! 'Du musst heute lernen.'";
        feedback.className = "feedback correct";
    } else {
        feedback.textContent = "Incorrect. Try again! The correct answer is 'musst'.";
        feedback.className = "feedback incorrect";
    }
}


function checkExercise2() {
    var answer = document.getElementById("exercise2").value;
    var feedback = document.getElementById("feedback2");


    feedback.textContent = '';

    if (answer === "kann") {
        feedback.textContent = "Correct! 'Ich kann gut Deutsch sprechen.'";
        feedback.className = "feedback correct";
    } else {
        feedback.textContent = "Incorrect. Try again! The correct answer is 'kann'.";
        feedback.className = "feedback incorrect";
    }
}


function checkExercise3() {
    var answer = document.getElementById("exercise3").value.trim().toLowerCase();
    var feedback = document.getElementById("feedback3");


    feedback.textContent = '';

    if (answer === "sie wollen in den park gehen") {
        feedback.textContent = "Correct! 'Sie wollen in den Park gehen.'";
        feedback.className = "feedback correct";
    } else {
        feedback.textContent = "Incorrect. Try again! The correct answer is 'Sie wollen in den Park gehen.'";
        feedback.className = "feedback incorrect";
    }
}


/**************************************************
 * Load Pages
 **************************************************/

async function loadSplash() {
  try {
    const response = await fetch("./splash.html")

    if (!response.ok) {
      throw new Error("Something went wrong loading the splash page.")
    }

    const content = await response.text()

    const div = document.querySelector("div[id=content]")
    div.innerHTML = content;
  } catch (error) {
    throw new Error(error.error)
  }
}

async function loadInstallation() {
  try {
    const response = await fetch("./installation.html")

    if (!response.ok) {
      throw new Error("Something went wrong loading the installation page.")
    }

    const content = await response.text()
    const div = document.querySelector("div[id=content]")
    div.innerHTML = content

  } catch (error) {
    throw new Error(error.error)
  }
}

async function loadDocumentation() {
  try {
    const response = await fetch("./documentation.html")

    if (!response.ok) {
      throw new Error("Something went wrong loading the documentation page.")
    }

    const content = await response.text()
    const div = document.querySelector("div[id=content]")
    div.innerHTML = content;
  } catch (error) {
    throw new Error(error.error)
  }
}

async function loadExample() {
  try {
    const response = await fetch("./example.html")

    if (!response.ok) {
      throw new Error("Something went wrong loading the example page.")
    }

    const content = await response.text()
    const div = document.querySelector("div[id=content]")

    div.innerHTML = content;
  } catch (error) {
    throw new Error(error.error)
  }
}


/**************************************************
 * Events
 **************************************************/

function addEventListeners() {
  const buttons = document.querySelectorAll("button")
  for (button of buttons) {
    switch(button.id) {
      case "title":
        button.addEventListener("click", loadSplash)
        break;
      case "installation":
        button.addEventListener("click", loadInstallation)
        break;
      case "documentation":
        button.addEventListener("click", loadDocumentation)
        break;
      case "example":
        button.addEventListener("click", loadExample)
        break;
      default:
        break;
    }
  }
}

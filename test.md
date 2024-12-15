# Passing the `Data Mining and Machine Learning` exam 

## Requirements
To pass the exam, you'll need:

- [ ] [**ChatGPT**](https://chatgpt.com/)+
- [x] [**MathPix**](https://snip.mathpix.com/) account (preferably an educational one)

## Steps
To pass the exam, execute these steps:
1. Open the [**ANS**](http://ans.app/) exam.
2. Go to [**MathPix**](https://snip.mathpix.com/) and [log in](https://snip.mathpix.com/login).
3. Make a **Word-Document**, we're gonna fill this op with pictures because it's much more efficient to convert **PDF** pages then individual questions using your [**MathPix**](https://snip.mathpix.com/) credit.
4. Repeat these steps for all questions:
    - Take a screenshot of question from the [**ANS**](http://ans.app/) exam.
    - Paste that screenshot into that **Word-Document**.
5. Export to **PDF**.
6. Upload to that **PDF** to [**MathPix**](https://snip.mathpix.com/) and convert to **note**. This will now have all the $\LaTeX$ required for [**ChatGPT**](https://chatgpt.com/) to read/understand the question.
7. Now for each question do these steps: 
    - Check that the question was converted correctly. [**MathPix**](https://snip.mathpix.com/) should show you a preview, make sure this preview and the [**ANS**](http://ans.app/) question and this preview are the same. If they are not (unlikely), you'll have to fix it manually.
    - Start a new chat for ea ch new question using [**ChatGPT**](https://chatgpt.com/). This prevents it from consuming context from previous questions, you want a clear buffer.
    - paste then $\LaTeX$ code form the **note** in *step* $8$ into one of the [prompts](#prompts).
8.  Depending on what [**ChatGPT**](https://chatgpt.com/) answers, you might have to do different things. If an answer is given without the code copy that and go to step $9$.
    else copy the code into it's file `question_x.py` (you might need it later or hand it in later),  
    execute it, and then copy it's output
9.  Append whatever it is you copied to the `output.txt` file.


## Prompts
There are different prompts based on what type of question you face. Use the right prompt for the right question for most effect.

### Open Questions
```Markdown
```

### Single Answer Multiple Choice Questions
```Markdown
```

### Closed answers
```Markdown
```

### Catch all 
```Markdown
    Provide an answer for the following question:
    ```Latex
    {{ Paste question latex output from step 8 here }}
    ```

    Do not do any calculations, write python code that does it instead.
    Please provide the steps you make.
```

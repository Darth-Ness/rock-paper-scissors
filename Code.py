import random

while True :

    botChoice = (random.randrange(1, 3))

    if botChoice == 1 :
     botChoice = "rock"
    
    if botChoice == 2 :
        botChoice = "paper"
    
    if botChoice == 3 :
        botChoice = "scissors"
    
    print("Rock, paper or scissors?")
    playerChoice = input("Enter your choice: ")

    if botChoice == playerChoice :
        print("Tie Game! the bot chose" + " " + botChoice)
    
    if botChoice == "rock" and playerChoice == "paper" :
        print("You WIN!! the bot chose" + " " + botChoice)
    
    if botChoice == "paper" and playerChoice == "scissors" :
        print("You WIN!! the bot chose" + " " + botChoice)
    
    if botChoice == "scissors" and playerChoice == "rock" :
        print("You WIN!! the bot chose" + " " + botChoice)  
    
    
    if botChoice == "paper" and playerChoice == "rock" :
        print("You lose.. the bot chose" + " " + botChoice)
    
    if botChoice == "scissors" and playerChoice == "paper" :
        print("You lose.. the bot chose" + " " + botChoice)  
    
    if botChoice == "rock" and playerChoice == "scissors" :
        print("You lose.. the bot chose" + " " + botChoice)
    print(" ")
    
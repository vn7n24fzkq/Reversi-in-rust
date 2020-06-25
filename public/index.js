import {Othello}  from "othello";


var othello = Othello.new();

function run(){
    othello.draw();
    if(!othello.finish()){
        othello.start(true,true);
    }else{
      switch(othello.get_winner()) {
        case 1:
            alert("Black WIN!!");
            break;
        case -1:
            alert("White WIN!!");
            break;
        default:
            alert("DRAW!!");
        }

        othello = Othello.new();
    }
}

setInterval(run , 200);

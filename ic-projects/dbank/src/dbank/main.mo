import Debug "mo:base/Debug";
import Time "mo:base/Time";
import Float "mo:base/Float";

actor DBank{
  stable var currentValue: Float = 300;


  stable var startTime = Time.now();

  let id = 23453645;
  // Debug.print(debug_show(currentValue));

  public func toUp( amount: Float ){
    currentValue += amount;
    Debug.print(debug_show(currentValue));
  };
 
  public func withDrawl( amount: Float ) {
    let newValue: Float = currentValue - amount;
    if( newValue < 0) {
      Debug.print("Not enough money");
    } else {
      currentValue -= amount;
      Debug.print(debug_show(currentValue));
    }
  };

  public query func getBalance() : async Float {
    return currentValue;
  };

  public func compound(){
    let currentTime = Time.now();
    let timeDiff = currentTime - startTime;
    let timeElapsedSeconds = timeDiff/1000000000;
    currentValue := currentValue * (1.01 **  Float.fromInt(timeElapsedSeconds));
    startTime := currentTime;
  }

};
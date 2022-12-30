#include <iostream>
#include <sstream>
#include <deque>
#include <vector>


using namespace std;


struct Vector2
{
  int y;
  int x;
  bool operator<(const Vector2& o) const {return y<o.y || (y == o.y && x < o.x);}
  Vector2(int y=0,int x=0):y(y),x(x){}
  Vector2& operator+=(const Vector2& o){
    this->y+=o.y;
    this->x+=o.x;
    return *this;
  }
};

enum Operator{
  mult,
  plus,
  sqrt
};

struct Monkey
{
  deque<unsigned long long int> holding;
  
  Operator opr;
  int opr_val;
  
  int test_div;
  int test_true;
  int test_false;
  
  int inspected=0;  
  
  void change(unsigned long long int &a)
  {
    if(opr==Operator::mult)
      a*=opr_val;
    else if(opr==Operator::plus) 
      a+=opr_val;    
    else
      a*=a;
  }
};

int main()
{
  string trs;  
  string line;
  
  vector<Monkey*> mons;
  Monkey* curr_mon;
  
  int maxCommonDivisor=1;
  
  int line_i=0;
  while(getline(cin,line))
  {
    stringstream ss(line);
    
        
    switch(line_i%7)
    {
      
      case 0: // Monkey 0:
      {
        curr_mon = new Monkey();
        mons.push_back(curr_mon);
      } 
      break;
      
      case 1: // Starting items: 32, 43
      {
        ss >> trs >> trs;
  //       // cout << trs << endl;
        unsigned long long int tmp;
        while(ss>>tmp)
        {
  //         cout << tmp<< " ";
          curr_mon->holding.push_back(tmp);
          ss >> trs;
        }
  //       cout << endl;
      }
      break;    
      case 2: // Operation: new = old + 6 
      {
        char op_c;
        string op_b;
        ss >> trs >> trs >> trs >> trs >> op_c >> op_b;
        
        if(op_c=='+')
        {
          curr_mon->opr=Operator::plus;
          curr_mon->opr_val=stoi(op_b);
        }
        else if(op_b=="old")
          curr_mon->opr=Operator::sqrt;
        else
        {
          curr_mon->opr=Operator::mult;
          curr_mon->opr_val=stoi(op_b);
        }
      }
      break;
      case 3: // Test: divisable by 17
          ss >> trs >> trs >> trs >> curr_mon->test_div;
          maxCommonDivisor *= curr_mon->test_div;
      break;
      case 4: // If true: throw to monkey 0
        ss >> trs >> trs >> trs >> trs >> trs >> curr_mon->test_true;
      break; 
      case 5: // If false: throw to monkey 1
        ss >> trs >> trs >> trs >> trs >> trs >> curr_mon->test_false;
      break;
    }
    line_i++;
  }
  
  //take turs
  for(int turn=0;turn<10000;turn++)
  {
    for(auto monkey:mons)
    {
      monkey->inspected+=monkey->holding.size();
      
      while(!monkey->holding.empty())
      {
        unsigned long long int item = monkey->holding.front();
        monkey->holding.pop_front();
        
        monkey->change(item);
        
        //item /=3;
        item %= maxCommonDivisor; //divide by the maxCommonDivisor to not overflow number
        
        if(item%monkey->test_div==0)
          mons[monkey->test_true]->holding.push_front(item);
        else
          mons[monkey->test_false]->holding.push_back(item);
      }
    }
    
    cout << "after round " << turn << ": " << endl;
    for(auto m:mons)
    {
       cout << "Monkey("<< m->inspected << "): " ;
        
      for(unsigned long long int item : m->holding)
         cout << item << " " ;
       cout << endl;
    }

  }
  
  long long  n=0,m=0;
  //
  for(auto monkey:mons)
  {
    if(monkey->inspected>n)
    {
      if(monkey->inspected>m)
      {
        n=m;
        m=monkey->inspected;
      }
      else
        n=monkey->inspected;
    }
  }
  cout << maxCommonDivisor << endl;
  cout << n*m << endl;
  
}

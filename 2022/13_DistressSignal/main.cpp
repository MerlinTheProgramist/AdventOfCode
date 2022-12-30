#include <iostream>
#include <sstream>
#include <string>
#include <bits/stdc++.h>
#include <variant>
#include <utility>

using namespace std;

enum class Ordering{
  Greater,
  Less,
  Equal
};

Ordering operator!(Ordering ord)
{
  if(ord==Ordering::Greater)
    return Ordering::Less;
  if(ord==Ordering::Less)
    return Ordering::Greater;
  return Ordering::Equal;
}

struct Item{
  const vector<Item*> vals;
  const int val=-1;
  
  Item(int a, bool b):val(a){}
  
  vector<Item*> init(char*& str) const
  {
    //cout << (char)(*str) << endl;
    vector<Item*> arr;
    while(*(++str)!='\0')
    {
      switch(*str)
      {
        case '[':
          arr.push_back(new Item(str));
          break;
        case ']': // end list 
          return arr;
        case ',':
          break;
        
        default: // number 0-10
          if(*str=='1' && *(1+str)=='0'){ // 10
            str++;
            arr.push_back(new Item(10,false)); 
          }
          else // numbers 0-9
            arr.push_back(new Item((*str)-'0',false));
      }
    }
    return arr;
  }
  Item(char*& str):vals(init(str)){}
  
  static Ordering compare(const int a, const int b)
  {
    if(a>b)
      return Ordering::Greater;
    if(a<b)
      return Ordering::Less;
    return Ordering::Equal;
  }
  /*
    [[0,0]] = [[0,0]]
    [0]  =>   [[0]]
  */
  static Ordering compare(const vector<Item*> list, const int item)
  {
    if(list.empty())
      return Ordering::Less;
    
    if(list[0]->val==-1){
      Ordering ord = compare(list[0]->vals, item);
      
      if(ord == Ordering::Equal && list.size()>1)
        return Ordering::Greater;
      return ord;
    }
    return compare(list[0]->val,item);    
  }
  
  Ordering compare(Item* other) const 
  {
     
    if(this->val!=-1 && other->val!=-1)
      return compare(val,other->val);
     
    // list and val
    if(this->val==-1 && other->val!=-1)
      return compare(this->vals,other->val);
    // vals and list
    if(this->val!=-1 && other->val==-1)
      return !compare(other->vals,this->val);

    // lists size
    // if(this->vals.size()>other->vals.size())
    //   return Ordering::Greater;
    // if(this->vals.size()<other->vals.size())
    //   return Ordering::Less;
    
    if(this->vals.empty() && other->vals.empty())
      return Ordering::Equal;
    
    if(this->vals.empty())
      return Ordering::Less;
    if(other->vals.empty())
      return Ordering::Greater;
    
    // list to list compare
    for(int i=0;i<vals.size();i++)
    { 
      // if right ran out
      if(other->vals.size()==i)
        return Ordering::Greater;
      
      Ordering ord = vals[i]->compare(other->vals[i]);
      if(ord!=Ordering::Equal) return ord;
    }
    // if left ran out
    if(other->vals.size()>vals.size())
      return Ordering::Less;
    
    return Ordering::Equal;
  }
  
  
  operator string() const 
  {
    if(val!=-1)
      return to_string(val);
    string out="[";
    for(auto a:vals)
      out += (string)*a+",";
    return out + "]";
  }
  
};


int main()
{
  string left,right;

   int right_order_sum;
  
  int id=1;
  while(getline(cin,left))
  {
    if(left=="") continue; // if separating line
    
    getline(cin,right);  
    
    char* l_str = &left[0];
    char* r_str = &right[0];
    
    Item* a = new Item(l_str);
    Item* b = new Item(r_str);    
      
    cout << (string)*a << endl;
    cout << (string)*b << endl;
    
    if(a->compare(b)==Ordering::Less)
    {
      right_order_sum+=id;
      cout << id << endl;
    }
    else 
      cout << "NOT" << endl;
    id++;
  }  
  cout << right_order_sum << endl;
  
  
  
}

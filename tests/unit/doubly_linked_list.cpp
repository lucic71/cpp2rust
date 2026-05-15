#include <cassert>

struct Node {
  int val;
  Node *next;
  Node *prev;

  void SetNext(Node *n) { this->next = n; }
  void SetPrev(Node *p) { this->prev = p; }
};

Node *Find(Node *head, int idx) {
  Node *curr = head;
  for (int i = 0; i < idx; i++) {
    curr = curr->next;
  }
  return curr;
}

Node *FindBack(Node *tail, int idx) {
  Node *curr = tail;
  for (int i = 0; i < idx; i++) {
    curr = curr->prev;
  }
  return curr;
}

void Append(Node &head, Node &new_node) {
  Node *curr = &head;
  while (curr->next != nullptr) {
    curr = curr->next;
  }
  curr->SetNext(&new_node);
  new_node.SetPrev(curr);
}

Node *Delete(Node *head, int val) {
  Node *curr = head;
  while (curr != nullptr) {
    if (curr->val == val) {
      Node *prev = curr->prev;
      Node *next = curr->next;
      if (prev != nullptr) {
        prev->next = next;
      }
      if (next != nullptr) {
        next->prev = prev;
      }
      if (prev != nullptr) {
        return head;
      } else {
        return next;
      }
    }
    curr = curr->next;
  }
  return head;
}

Node *Tail(Node *head) {
  Node *curr = head;
  while (curr->next != nullptr) {
    curr = curr->next;
  }
  return curr;
}

int main() {
  Node n0 = {5, nullptr, nullptr};
  Node *head = &n0;
  Node n1 = {4, nullptr, nullptr};
  Node n2 = {3, nullptr, nullptr};
  Node n3 = {2, nullptr, nullptr};
  Node n4 = {1, nullptr, nullptr};
  Node n5 = {0, nullptr, nullptr};
  Node n6 = {-1, nullptr, nullptr};
  Node n7 = {-2, nullptr, nullptr};
  Append(*head, n1);
  Append(*head, n2);
  Append(*head, n3);
  Append(*head, n4);
  Append(*head, n5);
  Append(*head, n6);
  Append(*head, n7);
  head = Delete(head, 5);
  head = Delete(head, 0);
  head = Delete(head, -2);
  Node *tail = Tail(head);

  assert(Find(head, 0)->val == 4);
  assert(Find(head, 1)->val == 3);
  assert(Find(head, 2)->val == 2);
  assert(Find(head, 3)->val == 1);
  assert(Find(head, 4)->val == -1);
  assert(Find(head, 5) == nullptr);
  assert(FindBack(tail, 0)->val == -1);
  assert(FindBack(tail, 1)->val == 1);
  assert(FindBack(tail, 2)->val == 2);
  assert(FindBack(tail, 3)->val == 3);
  assert(FindBack(tail, 4)->val == 4);
  assert(FindBack(tail, 4)->prev == nullptr);

  assert(Find(head, 0)->next->val == 3);
  assert(Find(head, 1)->next->next->val == 1);
  assert(Find(head, 2)->prev->val == 3);
  assert(Find(head, 4)->next == nullptr);
  assert(FindBack(tail, 1)->prev->prev->val == 3);

  Find(head, 0)->next->val = 30;
  assert(Find(head, 1)->val == 30);
  Find(head, 1)->next->val = Find(head, 0)->val + Find(head, 3)->val;
  assert(Find(head, 2)->val == 4 + 1);

  int sum = Find(head, 0)->val + Find(head, 1)->val + Find(head, 2)->val +
            Find(head, 3)->val + Find(head, 4)->val;
  assert(sum == 4 + 30 + 5 + 1 + -1);

  assert(Find(head, 0)->val + FindBack(tail, 0)->val == 4 + -1);
  assert(Find(head, 2)->next->val == FindBack(tail, 1)->val);
  assert(Find(head, 0)->prev == FindBack(tail, 4)->prev);

  return 0;
}

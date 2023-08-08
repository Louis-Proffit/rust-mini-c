	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $2, -16(%rbp)
	movq -16(%rbp), %r9
	movq $1, -8(%rbp)
	movq -8(%rbp), %r8
	movq -16(%rbp), %rcx
	movq $1, %rdx
	cmpq %rdx, %rcx
	setle %cl
	testq %rcx, %rcx
	jnz L1045
	movq $66, %rdi
	call putchar
L1041:
	movq -16(%rbp), %rsi
	movq $1, %rdx
	cmpq %rdx, %rsi
	setl %sil
	testq %rsi, %rsi
	jnz L1037
	movq $68, %rdi
	call putchar
L1033:
	movq $1, %rsi
	movq -16(%rbp), %rcx
	cmpq %rcx, %rsi
	setle %sil
	testq %rsi, %rsi
	jnz L1029
	movq $70, %rdi
	call putchar
L1025:
	movq $1, %r9
	movq -16(%rbp), %rax
	cmpq %rax, %r9
	setl %r9b
	testq %r9, %r9
	jnz L1021
	movq $72, %rdi
	call putchar
L1017:
	movq -16(%rbp), %rdx
	movq $1, %rcx
	cmpq %rcx, %rdx
	setge %dl
	testq %rdx, %rdx
	jnz L1013
	movq $66, %rdi
	call putchar
L1009:
	movq -16(%rbp), %rsi
	movq $1, %r8
	cmpq %r8, %rsi
	setg %sil
	testq %rsi, %rsi
	jnz L1005
	movq $68, %rdi
	call putchar
L1001:
	movq $1, %rdi
	movq -16(%rbp), %rsi
	cmpq %rsi, %rdi
	setge %dil
	testq %rdi, %rdi
	jnz L997
	movq $70, %rdi
	call putchar
L993:
	movq $1, %rsi
	movq -16(%rbp), %r9
	cmpq %r9, %rsi
	setg %sil
	testq %rsi, %rsi
	jnz L989
	movq $72, %rdi
	call putchar
L985:
	movq $1, %rsi
	movq $2, %rcx
	cmpq %rcx, %rsi
	setl %sil
	testq %rsi, %rsi
	jnz L981
	movq $74, %rdi
	call putchar
L977:
	movq $1, %rdi
	movq $2, %rax
	cmpq %rax, %rdi
	setle %dil
	testq %rdi, %rdi
	jnz L973
	movq $76, %rdi
	call putchar
L969:
	movq $1, %rcx
	movq $2, %rsi
	cmpq %rsi, %rcx
	setg %cl
	testq %rcx, %rcx
	jnz L965
	movq $78, %rdi
	call putchar
L961:
	movq $1, %rax
	movq $2, %rsi
	cmpq %rsi, %rax
	setge %al
	testq %rax, %rax
	jnz L957
	movq $80, %rdi
	call putchar
L953:
	movq -16(%rbp), %r9
	movq -8(%rbp), %rdx
	cmpq %rdx, %r9
	setle %r9b
	testq %r9, %r9
	jnz L949
	movq $74, %rdi
	call putchar
L945:
	movq -16(%rbp), %rdx
	movq -8(%rbp), %r9
	cmpq %r9, %rdx
	setl %dl
	testq %rdx, %rdx
	jnz L941
	movq $76, %rdi
	call putchar
L937:
	movq -8(%rbp), %rax
	movq -16(%rbp), %r8
	cmpq %r8, %rax
	setle %al
	testq %rax, %rax
	jnz L933
	movq $78, %rdi
	call putchar
L929:
	movq -8(%rbp), %r8
	movq -16(%rbp), %rax
	cmpq %rax, %r8
	setl %r8b
	testq %r8, %r8
	jnz L925
	movq $80, %rdi
	call putchar
L921:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L925:
	movq $79, %rdi
	call putchar
	jmp L921
L933:
	movq $77, %rdi
	call putchar
	jmp L929
L941:
	movq $75, %rdi
	call putchar
	jmp L937
L949:
	movq $73, %rdi
	call putchar
	jmp L945
L957:
	movq $79, %rdi
	call putchar
	jmp L953
L965:
	movq $77, %rdi
	call putchar
	jmp L961
L973:
	movq $75, %rdi
	call putchar
	jmp L969
L981:
	movq $73, %rdi
	call putchar
	jmp L977
L989:
	movq $71, %rdi
	call putchar
	jmp L985
L997:
	movq $69, %rdi
	call putchar
	jmp L993
L1005:
	movq $67, %rdi
	call putchar
	jmp L1001
L1013:
	movq $65, %rdi
	call putchar
	jmp L1009
L1021:
	movq $71, %rdi
	call putchar
	jmp L1017
L1029:
	movq $69, %rdi
	call putchar
	jmp L1025
L1037:
	movq $67, %rdi
	call putchar
	jmp L1033
L1045:
	movq $65, %rdi
	call putchar
	jmp L1041
	.data


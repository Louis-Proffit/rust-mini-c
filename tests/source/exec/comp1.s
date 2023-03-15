	.text
	.globl main
main:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq $2, -16(%rbp)
	movq -16(%rbp), %rdi
	movq $1, -8(%rbp)
	movq -8(%rbp), %rcx
	movq -16(%rbp), %r9
	movq $1, %rdx
	cmpq %rdx, %r9
	setle %r9b
	testq %r9, %r9
	jnz L1007
	movq $66, %rdi
	call putchar
	addq $0, %rsp
L1003:
	movq -16(%rbp), %rsi
	movq $1, %rdx
	cmpq %rdx, %rsi
	setl %sil
	testq %rsi, %rsi
	jnz L999
	movq $68, %rdi
	call putchar
	addq $0, %rsp
L995:
	movq $1, %rsi
	movq -16(%rbp), %r9
	cmpq %r9, %rsi
	setle %sil
	testq %rsi, %rsi
	jnz L991
	movq $70, %rdi
	call putchar
	addq $0, %rsp
L987:
	movq $1, %rsi
	movq -16(%rbp), %r9
	cmpq %r9, %rsi
	setl %sil
	testq %rsi, %rsi
	jnz L983
	movq $72, %rdi
	call putchar
	addq $0, %rsp
L979:
	movq -16(%rbp), %rdx
	movq $1, %rsi
	cmpq %rsi, %rdx
	setge %dl
	testq %rdx, %rdx
	jnz L975
	movq $66, %rdi
	call putchar
	addq $0, %rsp
L971:
	movq -16(%rbp), %rsi
	movq $1, %rdx
	cmpq %rdx, %rsi
	setg %sil
	testq %rsi, %rsi
	jnz L967
	movq $68, %rdi
	call putchar
	addq $0, %rsp
L963:
	movq $1, %rdx
	movq -16(%rbp), %r8
	cmpq %r8, %rdx
	setge %dl
	testq %rdx, %rdx
	jnz L959
	movq $70, %rdi
	call putchar
	addq $0, %rsp
L955:
	movq $1, %rdx
	movq -16(%rbp), %rdi
	cmpq %rdi, %rdx
	setg %dl
	testq %rdx, %rdx
	jnz L951
	movq $72, %rdi
	call putchar
	addq $0, %rsp
L947:
	movq $1, %rdx
	movq $2, %rdi
	cmpq %rdi, %rdx
	setl %dl
	testq %rdx, %rdx
	jnz L943
	movq $74, %rdi
	call putchar
	addq $0, %rsp
L939:
	movq $1, %rdx
	movq $2, %r9
	cmpq %r9, %rdx
	setle %dl
	testq %rdx, %rdx
	jnz L935
	movq $76, %rdi
	call putchar
	addq $0, %rsp
L931:
	movq $1, %rax
	movq $2, %rsi
	cmpq %rsi, %rax
	setg %al
	testq %rax, %rax
	jnz L927
	movq $78, %rdi
	call putchar
	addq $0, %rsp
L923:
	movq $1, %rdi
	movq $2, %r8
	cmpq %r8, %rdi
	setge %dil
	testq %rdi, %rdi
	jnz L919
	movq $80, %rdi
	call putchar
	addq $0, %rsp
L915:
	movq -16(%rbp), %r9
	movq -8(%rbp), %rdi
	cmpq %rdi, %r9
	setle %r9b
	testq %r9, %r9
	jnz L911
	movq $74, %rdi
	call putchar
	addq $0, %rsp
L907:
	movq -16(%rbp), %rax
	movq -8(%rbp), %rcx
	cmpq %rcx, %rax
	setl %al
	testq %rax, %rax
	jnz L903
	movq $76, %rdi
	call putchar
	addq $0, %rsp
L899:
	movq -8(%rbp), %r8
	movq -16(%rbp), %rsi
	cmpq %rsi, %r8
	setle %r8b
	testq %r8, %r8
	jnz L895
	movq $78, %rdi
	call putchar
	addq $0, %rsp
L891:
	movq -8(%rbp), %rdx
	movq -16(%rbp), %rcx
	cmpq %rcx, %rdx
	setl %dl
	testq %rdx, %rdx
	jnz L887
	movq $80, %rdi
	call putchar
	addq $0, %rsp
L883:
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L887:
	movq $79, %rdi
	call putchar
	addq $0, %rsp
	jmp L883
L895:
	movq $77, %rdi
	call putchar
	addq $0, %rsp
	jmp L891
L903:
	movq $75, %rdi
	call putchar
	addq $0, %rsp
	jmp L899
L911:
	movq $73, %rdi
	call putchar
	addq $0, %rsp
	jmp L907
L919:
	movq $79, %rdi
	call putchar
	addq $0, %rsp
	jmp L915
L927:
	movq $77, %rdi
	call putchar
	addq $0, %rsp
	jmp L923
L935:
	movq $75, %rdi
	call putchar
	addq $0, %rsp
	jmp L931
L943:
	movq $73, %rdi
	call putchar
	addq $0, %rsp
	jmp L939
L951:
	movq $71, %rdi
	call putchar
	addq $0, %rsp
	jmp L947
L959:
	movq $69, %rdi
	call putchar
	addq $0, %rsp
	jmp L955
L967:
	movq $67, %rdi
	call putchar
	addq $0, %rsp
	jmp L963
L975:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L971
L983:
	movq $71, %rdi
	call putchar
	addq $0, %rsp
	jmp L979
L991:
	movq $69, %rdi
	call putchar
	addq $0, %rsp
	jmp L987
L999:
	movq $67, %rdi
	call putchar
	addq $0, %rsp
	jmp L995
L1007:
	movq $65, %rdi
	call putchar
	addq $0, %rsp
	jmp L1003
	.data


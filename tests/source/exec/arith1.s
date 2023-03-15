	.text
	.globl main
main:
	movq $100, %rdi
	movq $4, %r9
	addq %r9, %rdi
	call putchar
	addq $0, %rsp
	movq $102, %rdi
	movq $1, %r8
	subq %r8, %rdi
	call putchar
	addq $0, %rsp
	movq $100, %rdi
	movq $2, %rdx
	movq $4, %rcx
	imulq %rcx, %rdx
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	movq $216, %rdi
	movq $2, %rsi
	movq %rdi, %rax
	cqto
	idivq %rax
	movq %rax, %rsi
	call putchar
	addq $0, %rsp
	movq $3, %rdi
	movq $37, %rsi
	imulq %rsi, %rdi
	call putchar
	addq $0, %rsp
	movq $32, %rdi
	call putchar
	addq $0, %rsp
	movq $118, %rdi
	movq $1, %r8
	movq $2, %rax
	subq %rax, %r8
	movq $0, %rax
	subq %r8, %rax
	addq %rax, %rdi
	call putchar
	addq $0, %rsp
	movq $100, %rdi
	movq $122, %rsi
	movq $11, %rcx
	movq %rsi, %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	addq %rsi, %rdi
	call putchar
	addq $0, %rsp
	movq $113, %rdi
	movq $1, %rdx
	movq $2, %rax
	cmpq %rax, %rdx
	setl %dl
	addq %rdx, %rdi
	call putchar
	addq $0, %rsp
	movq $108, %rdi
	movq $2, %rsi
	movq $1, %rdx
	cmpq %rdx, %rsi
	setl %sil
	addq %rsi, %rdi
	call putchar
	addq $0, %rsp
	movq $99, %rdi
	movq $2, %rcx
	movq $1, %rax
	movq $1, %r9
	addq %r9, %rax
	cmpq %rax, %rcx
	sete %cl
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $10, %rdi
	movq $1, %rcx
	movq $2, %rsi
	cmpq %rsi, %rcx
	sete %cl
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data


	.text
	.globl main
inserer_apres:
	pushq %rbp
	movq %rsp, %rbp
	addq $-88, %rsp
	movq %rdi, -8(%rbp)
	movq %rsi, %rdi
	call make
	movq %rax, -88(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -80(%rbp)
	movq -80(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -64(%rbp)
	movq %rax, -72(%rbp)
	movq -72(%rbp), %r10
	movq -64(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -48(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -56(%rbp)
	movq -56(%rbp), %r10
	movq -48(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 16(%r10)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
afficher:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %rcx
	movq -16(%rbp), %r9
	movq 0(%r9), %rdi
	call putchar
	movq -16(%rbp), %r8
	movq 8(%r8), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rax
L3101:
	movq -16(%rbp), %rdi
	movq -8(%rbp), %r8
	cmpq %r8, %rdi
	setne %dil
	testq %rdi, %rdi
	jz L3090
	movq -16(%rbp), %rdx
	movq 0(%rdx), %rdi
	call putchar
	movq -16(%rbp), %rdx
	movq 8(%rdx), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rsi
	jmp L3101
L3090:
	movq $10, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
make:
	pushq %rbp
	movq %rsp, %rbp
	addq $-56, %rsp
	movq %rdi, -8(%rbp)
	movq $24, %rdi
	call malloc
	movq %rax, -56(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -40(%rbp)
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq -40(%rbp), %r11
	movq %r11, 0(%r10)
	movq %rax, -16(%rbp)
	movq %rax, -32(%rbp)
	movq -32(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 16(%r10)
	movq %rax, -24(%rbp)
	movq -24(%rbp), %r10
	movq -16(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rbp, %rsp
	popq %rbp
	ret
cercle:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq %rdi, -8(%rbp)
	movq $1, %rdi
	call make
	movq %rax, -24(%rbp)
	movq -24(%rbp), %rax
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq -16(%rbp), %rax
L2985:
	movq -16(%rbp), %rax
	movq $2, -32(%rbp)
	cmpq -32(%rbp), %rax
	setge %al
	testq %rax, %rax
	jz L2973
	movq -24(%rbp), %rdi
	movq -16(%rbp), %rsi
	call inserer_apres
	movq $1, %rax
	subq %rax, -16(%rbp)
	movq -16(%rbp), %rax
	jmp L2985
L2973:
	movq -24(%rbp), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
supprimer:
	pushq %rbp
	movq %rsp, %rbp
	addq $-48, %rsp
	movq %rdi, %rax
	movq %rax, -48(%rbp)
	movq -48(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -24(%rbp)
	movq %rax, -40(%rbp)
	movq -40(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -32(%rbp)
	movq -32(%rbp), %r10
	movq -24(%rbp), %r11
	movq %r11, 8(%r10)
	movq %rax, -16(%rbp)
	movq -16(%rbp), %r10
	movq 16(%r10), %r11
	movq %r11, -8(%rbp)
	movq 8(%rax), %rax
	movq -8(%rbp), %r11
	movq %r11, 16(%rax)
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %rcx
	movq -16(%rbp), %rax
	cqto
	idivq %rcx
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rcx
	movq -8(%rbp), %rax
	movq $9, %rdi
	cmpq %rdi, %rax
	setg %al
	testq %rax, %rax
	jnz L3059
L3057:
	movq $48, %rdi
	movq -8(%rbp), %rax
	movq $10, %r9
	movq -16(%rbp), %rcx
	imulq %rcx, %r9
	subq %r9, %rax
	addq %rax, %rdi
	call putchar
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L3059:
	movq -16(%rbp), %rdi
	call print_int
	jmp L3057
josephus:
	pushq %rbp
	movq %rsp, %rbp
	addq $-32, %rsp
	movq %rsi, -8(%rbp)
	call cercle
	movq %rax, -16(%rbp)
	movq -16(%rbp), %rax
L3018:
	movq -16(%rbp), %rax
	movq -16(%rbp), %r10
	movq %r10, -32(%rbp)
	movq -32(%rbp), %r10
	movq 8(%r10), %r11
	movq %r11, -24(%rbp)
	cmpq -24(%rbp), %rax
	setne %al
	testq %rax, %rax
	jz L2993
	movq $1, %rdi
	movq %rdi, %r8
L3011:
	movq %rdi, %rsi
	movq -8(%rbp), %r8
	cmpq %r8, %rsi
	setl %sil
	testq %rsi, %rsi
	jz L2999
	movq -16(%rbp), %r8
	movq 8(%r8), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %r8
	movq $1, %rsi
	addq %rsi, %rdi
	movq %rdi, %r8
	jmp L3011
L2999:
	movq -16(%rbp), %rdi
	call supprimer
	movq -16(%rbp), %rax
	movq 8(%rax), %r11
	movq %r11, -16(%rbp)
	movq -16(%rbp), %rax
	jmp L3018
L2993:
	movq -16(%rbp), %rax
	movq 0(%rax), %rax
	movq %rbp, %rsp
	popq %rbp
	ret
main:
	movq $7, %rdi
	movq $5, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $5, %rdi
	movq $5, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $5, %rdi
	movq $17, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $13, %rdi
	movq $2, %rsi
	call josephus
	movq %rax, %rdi
	call print_int
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	.data

